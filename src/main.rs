//                   GNU LESSER GENERAL PUBLIC LICENSE
//                        Version 2.1, February 1999
//
// farmtasker.au a marketplace website for local farmers in Tasmania.
// Copyright (C) 2024 Dmytro Serdiukov & FARMTASKER PTY LTD
//
// NOTE: All images, logos, and branding are the exclusive property of FARMTASKER PTY LTD and are not covered by the open-source license.
// These assets may not be used publically without prior written permission.
//
// This software is a free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//
// See the GNU Lesser General Public License for more details.
//
// FARMTASKER PDY LTD, hereby disclaims all copyright interest in the
// software `farmtasker.au' (a marketplace website for local farmers in Tasmania) written by Dmytro Serdiukov.
//
// You can contact us at info@farmtasker.au

#![allow(unused)]
#[cfg(not(feature = "ssr"))]
fn main() {}

pub use axum::routing::post;
pub use axum::*;
pub use core::panic;
pub use farmtasker_au::app::*;
pub use farmtasker_au::fileserv::file_and_error_handler;
pub use farmtasker_au::*;
pub use leptos::*;
pub use leptos_axum::{generate_route_list, LeptosRoutes};
pub use std::borrow::BorrowMut;
pub use std::io::{BufRead, BufReader};
pub use std::sync::{Arc, Mutex};
pub use stripe::{Metadata, *};
pub use tracing_subscriber;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::collections::HashMap;

    tracing_subscriber::fmt::init();

    tracing::info!("-------------------------------------------");
    tracing::info!("Starting up...");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let site_root = std::env::var("LEPTOS_SITE_ROOT").expect("Missing LEPTOS_SITE_ROOT variable in env.");

    tracing::info!("LEPTOS_SITE_ROOT={}", site_root);

    let key = std::env::var("STRIPE_KEY")
        .expect("Missing STRIPE_KEY variable in env. Please do 'export STRIPE_KEY=sk_*******'.\nYou can verify if env variable 'STRIPE_KEY' is present with 'env | grep STRIPE_KEY'");

    let stripe_client = stripe::Client::new(key.clone());

    let appstate = farmtasker_au::AppState {
        stripe_data: match farmtasker_au::StripeData::new_fetch().await {
            Ok(ok) => Some(ok),
            Err(err) => {
                leptos::logging::log!("No StripeData in AppState");
                None
            }
        },
        products_config: match fetch_local_product_info().await {
            Ok(ok) => Some(ok),
            Err(err) => {
                leptos::logging::log!("No local CfgProducts in AppState");
                None
            }
        }
    };

    refresh_local_product_info(false).await;
    tracing::info!("");
    
    assert!(
        &appstate.stripe_data.clone().is_some(), 
        "No StripeData in AppState during server init. Check if the StripeData could be fetched from internet.\nPlease verify that you have internet connection.",
    );
    assert!(
        &appstate.products_config.clone().is_some(), 
        "No CfgProducts in AppState during server init. Check if the products config could be initiated. How did this happen?",
    );

    let products = &appstate
        .stripe_data
        .clone()
        .unwrap()
        .products;
    tracing::info!("Listing products:");
    for i in products {
        tracing::info!(
            "#{:?} Product: {:#?} - {:#?}$ AUD",
            i.metadata.clone().unwrap_or(HashMap::new()).get("item_number").map_or("_", |v| v).parse().unwrap_or(-1),
            i.name,
            i.default_price.clone().unwrap().unit_amount.unwrap() as f64 / 100.0
        );
    }
    let customers = &appstate
        .stripe_data
        .clone()
        .unwrap()
        .customers;

    let checkout_sessions = &appstate
        .stripe_data
        .clone()
        .unwrap()
        .checkout_sessions
        .clone();
    tracing::info!("Total \"Products\": {:}", products.len());

    tracing::info!("Total \"Customers\": {:}", customers.len());

    tracing::info!(
        "Total of currently Open \"Checkout Sessions\": {:}",
        checkout_sessions
            .iter()
            .filter(|c| match &c.status {
                Some(s) => match s {
                    crate::stripe_retypes::DbCheckoutSessionStatus::Complete => false,
                    crate::stripe_retypes::DbCheckoutSessionStatus::Expired => false,
                    crate::stripe_retypes::DbCheckoutSessionStatus::Open => true,
                },
                None => false,
            })
            .collect::<Vec<&crate::stripe_retypes::DbCheckoutSession>>()
            .len()
    );

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let appstate = appstate.clone();
                move || provide_context(Some(appstate.clone()))
            },
            App,
        )
        .fallback(file_and_error_handler)
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .layer(Extension(appstate.clone()))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("-------------------------------------------");
    tracing::info!("Server started. listening on http://{}\n", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
