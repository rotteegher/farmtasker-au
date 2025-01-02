#![allow(unused)]
use crate::error_template::{AppError, ErrorTemplate};

use crate::*;
use leptos::*;
use leptos_dom::logging;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::*;
use leptos_use::*;
use log::*;

pub type StripeDataRes = Resource<(), Result<StripeData, ServerFnError>>;
pub type CfgProductsRes = Resource<(), Result<CfgProducts, ServerFnError>>;
pub type CheckoutSessionRes = Resource<i64, Result<DbCheckoutSession, ServerFnError>>;

pub type CheckoutSessionIdRes = String;

pub type CheckoutSessionUpdateRes = i64;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let stripe_data: StripeDataRes =
        create_resource(|| (), move |_| async { stripe_stater().await });
    provide_context(stripe_data);

    let products_config: CfgProductsRes =
        create_resource(|| (), move |_| async { products_stater().await });
    provide_context(stripe_data);

    let (current_page, set_current_page) = create_signal(CurrentPage::None);
    provide_context(current_page);
    provide_context(set_current_page);

    let (shopping_cart, set_shopping_cart, clear_shopping_cart) =
        use_local_storage_with_options::<ShoppingCart, codee::string::JsonSerdeCodec>(
            "shopping_cart",
            UseStorageOptions::default().delay_during_hydration(true),
        );
    provide_context(shopping_cart);
    provide_context(set_shopping_cart);
    provide_context(clear_shopping_cart);

    let (checkout_sessionid, set_checkout_sessionid, clear_checkout_sessionid) =
        use_local_storage_with_options::<CheckoutSessionIdRes, codee::string::JsonSerdeCodec>(
            "checkout_sessionid",
            UseStorageOptions::default().delay_during_hydration(true),
        );
    provide_context(checkout_sessionid);
    provide_context(set_checkout_sessionid);
    provide_context(clear_checkout_sessionid);

    let (submit_checkout, set_submit_checkout): (
        ReadSignal<CheckoutSessionUpdateRes>,
        WriteSignal<CheckoutSessionUpdateRes>,
    ) = create_signal::<CheckoutSessionUpdateRes>(0);
    provide_context(submit_checkout);
    provide_context(set_submit_checkout);

    let checkout_session: CheckoutSessionRes =
        create_resource(submit_checkout, move |x| async move {
            new_checkout_session(shopping_cart.get().clone().0, checkout_sessionid.get()).await
        });
    provide_context(checkout_session);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/farmtasker-au.css"/>

        // sets the document title
        <Title text="Farmtasker Shop"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <nav>
                <NavBar/>
            </nav>
            <main>
                <Routerer/>
            </main>
            <div>
                <FooterBar/>
            </div>
        </Router>
    }
}
#[component]
pub fn Routerer() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::HomePage;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=HomePage currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/pet" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::PetFood;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=PetFood currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/food" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::FarmFood;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=FarmFood currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/products/:product_name"  view={
                move || {
                    let CURRENTPAGE: CurrentPage = CurrentPage::DbProductItemDetails;

                    let params = use_params_map();
                    let product_name = params.with(|params| params.get("product_name").cloned()).unwrap_or("no parameter".into());


                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=move || {view!{<DbProductItemDetailsPage product_name=product_name.clone()/>}} currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/about" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::About;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=About currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/delivery" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::Delivery;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=Delivery currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/privacy" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::PrivacyPolicy;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=PrivacyPolicy currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/terms" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CurrentPage::TermsOfService);
                    view! {
                        <Pager page=TermsOfService currentpage=CurrentPage::TermsOfService/>
                    }
                }
            }/>
            <Route path="/shop/cart" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::ShoppingCart;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=ShoppingCart currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/instructions" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoInstructions;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoInstructions currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/blog/culinary-adventure" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::VideoBlogs;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=VideoBlogs currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/shop/eat" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::EatNow;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=EatNow currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/success" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=SuccessCheckout currentpage=CURRENTPAGE/>
                    }
                }
            }/>
            <Route path="/cancel" view={
                move || {
                    const CURRENTPAGE: CurrentPage = CurrentPage::None;

                    let setter = expect_context::<WriteSignal<CurrentPage>>();
                    setter.update(|page: &mut CurrentPage| *page = CURRENTPAGE);
                    view! {
                        <Pager page=CancelCheckout currentpage=CURRENTPAGE/>
                    }
                }
            }/>
        </Routes>
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CurrentPage {
    None,
    HomePage,
    PetFood,
    FarmFood,
    EatNow,
    About,
    Delivery,
    DbProductItemDetails,
    PrivacyPolicy,
    TermsOfService,
    ShoppingCart,
    VideoInstructions,
    VideoBlogs,
}

#[component]
pub fn Pager<F, IV>(page: F, currentpage: CurrentPage) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="page">
            <div class="pager-bg">
                <div class="pager">
                    <div
                        class="pager-content"
                        class=match currentpage {
                            CurrentPage::None => {"pager-content-none"},
                            CurrentPage::HomePage => {"pager-content-home-page"},
                            CurrentPage::PetFood => {"pager-content-pet-shop pager-content-shop-general"},
                            CurrentPage::FarmFood => {"pager-content-foot-shop pager-content-shop-general"},
                            CurrentPage::About => {"pager-content-about"},
                            CurrentPage::Delivery => {"pager-content-delivery"},
                            CurrentPage::PrivacyPolicy => {"pager-content-privacy-policy"},
                            CurrentPage::TermsOfService => {"pager-content-terms-of-service"},
                            CurrentPage::ShoppingCart => {"pager-content-shopping-cart"},
                            CurrentPage::VideoInstructions => {"pager-content-video-instructions"},
                            CurrentPage::VideoBlogs => {"pager-content-video-blogs"},
                            CurrentPage::DbProductItemDetails => {"pager-content-product-item-details"},
                            CurrentPage::EatNow => {"pager-content-eat-now-shop"},
                        }
                    >{page()}</div>
                </div>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="main_buttons_online_shops_container">
            <a href="/shop/food" class="page-selector-container" id="button_farm_to_table_near_me">
                <img class="page-selector-image" src="/main_buttons/food_shop.png" alt="Online Shop"/>
            </a>
            <a href="/shop/pet" class="page-selector-container" id="button_farmtasker_pet_food_shop">
                <img class="page-selector-image" src="/main_buttons/pet_food_shop.png" alt="Online Shop"/>
            </a>
        </div>
        <div class="main_buttons_services_container">
            <a href="/shop/eat" class="page-selector-container" id="button_ready_to_eat_shop">
                <img class="page-selector-image" src="/main_buttons/ready_to_eat_shop.png" alt="Eat Now"/>
            </a>
            <a href="/instructions" class="page-selector-container" id="button_farm_task_video_instructions_service">
                <img class="page-selector-image" src="/main_buttons/instructions.png" alt="Video Instructions"/>
            </a>
            <a href="/blog/culinary-adventure" class="page-selector-container" id="button_culinary_adventure">
                <img class="page-selector-image" src="/main_buttons/video_blog.png" alt="Video Blog"/>
            </a>
        </div>
    }
}

#[component]
pub fn DbProductItemDetails(product: DbProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    provide_context(product);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <div class="product-item-container">
            <Show
                when=move || {product.get().images.is_some_and(|x| !x.is_empty())}
                fallback=move || {view!{
                    <div class="product-item-empty">
                    </div>
                }}
            >
                <img class="product-item-image" src={product.get().images.unwrap().first()}/>
            </Show>
            <div class="product-info">
                <strong class="product-item-name">
                    {product.get().name}
                </strong>
                <p class="product-item-description">
                    {product.get().description.unwrap_or("No Description.".to_string())}
                </p>
            </div>
            <button class="product-item-addtocart-button" on:click=move |_| {
                set_shopping_cart.update(|s| {
                    s.add_single_product(&product.get().id, 20);
                });
            }>
            "Add To Cart $"{product.get().default_price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn DbProductItemDetailsPage(product_name: String) -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    let (product_name, _) = create_signal(product_name);
    provide_context(product_name);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match stripe_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(stripe_data) => {
                    let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let product_name = expect_context::<ReadSignal<String>>();
                    provide_context(product_name);


                    match stripe_data.products.into_iter()
                    .find(|product| {
                        let cmp1 = product.name.to_lowercase().replace(" ", "-");
                        let cmp2 = &product_name.get()[1..];

                        cmp1 == cmp2
                    }) {
                        Some(product) => {
                            view!{
                                <DbProductItemDetails product=product/>
                            }.into_view()
                        },
                        None => view!{
                            <div>"NO PRODUCT WITH SUCH NAME"</div>
                        }.into_view(),
                    }
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn PetFood() -> impl IntoView {
    view! {
        <h1 class="shop-title">"Pet Food Shop"</h1>
        <DbProductItemsList items_category="pet_food".to_string()/>
    }
}

#[component]
pub fn FarmFood() -> impl IntoView {
    view! {
        <h1 class="shop-title">"Farm Food Shop"</h1>
        <DbProductItemsList items_category="food".to_string()/>
    }
}

#[component]
pub fn VideoInstructions() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/instructions.webp" alt="Video Instructions Banner"/>
            <iframe class="embed-video" src="https://www.youtube.com/embed/daOJwUdwTME?si=GrQjf7z3ZiHD4WQF" title="Instructions" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
        </div>
    }
}

#[component]
pub fn EatNow() -> impl IntoView {
    view! {
        <div class="blog-container">
            // <img class="banner-image" src="/banners/.webp" alt="Ready To Eat Banner"/>
        </div>
    }
}

#[component]
pub fn VideoBlogs() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/video_blog.webp" alt="Video Blog Banner"/>
            <iframe class="embed-video" src="https://www.youtube.com/embed/EFyeoMRsDN8?si=pqqFHuqhTuB5xNMV" title="Culinary Adventure" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
        </div>
        <a href="/instructions" class="page-selector-container" id="button_farm_task_video_instructions">
            <img style="max-height: 20rem" class="page-selector-image" src="/main_buttons/instructions.png" alt="Video Instructions"/>
        </a>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/about_us_cropped.webp" alt="About Us Banner"/>
            <img class="banner-photo" src="/photos/about_us_group_photo.webp" alt="About Us Photo Banner"/>
        </div>
        <div class="about-us-blocks-container">
            <ul class="about-us-block">
                <h3>"Olesia – Director & Co-Founder (50% Farmtasker PTY LTD)"</h3>
                <h4>"With over 25 years of business and finance experience in Ukraine, Olesia is the driving force behind Farmtasker. She successfully launched her first online shop in 2004, showcasing her entrepreneurial vision."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"Bookkeeping in Australia: May 2022 – present"</li>
                        <li style="font-size: 16px">"Poultry processing expertise: December 2022 – present"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Master's Degree in Economics (2001)"</li>
                <h4>"Olesia’s leadership combines strategic insight with hands-on industry knowledge, ensuring the smooth operation of Farmtasker."</h4>
            </ul>
            <ul class="about-us-block">
                <h3>"Vasyl – Sales & Marketing Manager, Co-Founder (50% Farmtasker PTY LTD)"</h3>
                <h4>"Vasyl brings a wealth of expertise with 10+ years of experience in government roles, including the State Tax Service of Ukraine and Legal Advisory for Kyiv Consulate. His qualifications are officially recognized by the Australian Government."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"Sales & Business Management: Nearly 10 years of success in Ukraine"</li>
                        <li style="font-size: 16px">"Poultry processing expertise: December 2022 – present"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Master’s Degree in Finance (2007)"</li>
                        <li style="font-size: 16px">"Master’s Degree in Law (2011)"</li>
                <h4>"Vasyl’s strategic sales approach and legal experience ensure Farmtasker delivers with professionalism and integrity."</h4>
            </ul>
            <ul class="about-us-block">
                <h3>"Dmytro – Software Engineer, Farmtasker PTY LTD"</h3>
                <h4>"As the tech backbone of Farmtasker, Dmytro combines his technical prowess with creative skills."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"1.8 years as a Multimedia Officer, Future Digital Department, University of Tasmania"</li>
                        <li style="font-size: 16px">"Kitchen hand experience: Agrarian Kitchen, New Norfolk (December 2023 – December 2024)"</li>
                        <li style="font-size: 16px">"5 years of sound engineering and video editing experience"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Dmytro is currently pursuing his Bachelor’s Degree in Software Engineering (2025), playing a key role in building Farmtasker’s online presence and digital solutions."</li>
            </ul>
            <ul class="about-us-block">
                <h3>"Margo – Co-Founders’ Daughter & Future Vet"</h3>
                    <h4>"Born in 2012, Margo embodies the family’s next generation of passion and energy. A devoted animal lover, she aspires to become a veterinarian while actively contributing to family projects."</h4>
            </ul>
        </div>
    }
}

#[component]
pub fn Delivery() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <div class="privacy-policy">
            <h1>"Privacy Policy"</h1>
            <p>
                "Our Privacy Policy is currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:info@farmtasker.au">" info@farmtasker.au"</a>
                "."
            </p>
        </div>
    }
}

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <div class="terms-of-service">
            <h1>"Terms of Service"</h1>
            <p>
                "Our Terms of Service are currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:info@farmtasker.au">" info@farmtasker.au"</a>
                "."
            </p>
        </div>
    }
}

#[component]
pub fn DbProductItem(product: DbProduct) -> impl IntoView {
    let (product, _) = create_signal(product);
    provide_context(product);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);
    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    view! {
        <div class="product-item-container">
            <a href=move || {
                let product_name = product.get().name.to_lowercase().replace(" ", "-");
                format!("/shop/products/:{:#}", product_name)
            }>
                <Show
                    when=move || {product.get().images.is_some_and(|x| !x.is_empty())}
                    fallback=move || {view!{
                        <div class="product-item-empty">
                        </div>
                    }}
                >
                    <img class="product-item-image" src={product.get().images.unwrap().first()}/>
                </Show>
                <div class="product-info">
                    <strong class="product-item-name">
                        {product.get().name}
                    </strong>
                </div>
            </a>
            <button class="product-item-addtocart-button" on:click=move |_| {
                set_shopping_cart.update(|s| {
                    s.add_single_product(&product.get().id, 20);
                });
            }>
            "Add To Cart $"{product.get().default_price.unwrap().unit_amount.unwrap() / 100}
            </button>
        </div>
    }
}

#[component]
pub fn DbProductItemsList(items_category: String) -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    let (items_category, set_items_category) = create_signal(items_category);
    provide_context(items_category);

    view! {
        <Suspense fallback=move || view! {"loading data"}>
            {move || match stripe_data.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(stripe_data) => {
                    let stripe_data: StripeData = stripe_data.expect("Resource StripeData is not here on 'get()'");
                    let items_category = expect_context::<ReadSignal<String>>();
                    provide_context(items_category);


                    view! {
                        <ul class="product-list-ul">
                            {
                                stripe_data.products.into_iter()
                                .filter(|product| {
                                    product.metadata
                                        .as_ref()
                                        .and_then(|metadata| metadata.get("category"))
                                        .map(|category| category == &items_category.get())
                                        .unwrap_or(false)
                                })
                                .map(|product| {
                                    view! {
                                        <li class="product-list-item">
                                            <DbProductItem product=product/>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    }.into_view()
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn SuccessCheckout() -> impl IntoView {
    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    view! {
        <p>
            "Checkout completed successfully!"
        </p>
        <p>
            "You should find details of your order in your email soon."
        </p>
    }
}

#[component]
pub fn CancelCheckout() -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    provide_context(stripe_data);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);
    let set_checkout_sessionid = expect_context::<WriteSignal<CheckoutSessionIdRes>>();
    provide_context(set_checkout_sessionid);

    let submit_checkout = expect_context::<ReadSignal<CheckoutSessionUpdateRes>>();
    provide_context(submit_checkout);
    let set_submit_checkout = expect_context::<WriteSignal<CheckoutSessionUpdateRes>>();
    provide_context(set_submit_checkout);

    let checkout_session = expect_context::<CheckoutSessionRes>();
    provide_context(checkout_session);

    let loading = checkout_session.loading();
    let is_loading = move || {
        if loading() {
            "Loading..."
        } else {
            let session = checkout_session
                .get()
                .map(|value| {
                    value
                        .map(|value2| {
                            set_checkout_sessionid.update(|s| *s = value2.id.clone());
                            value2.id
                        })
                        .unwrap_or_else(|x| "Loading 2".into())
                })
                .unwrap_or_else(|| "Loading".into());
            // leptos::logging::log!("session: {:#?}", session);
            // stripe_data.get().expect("no stripdata lol")
            "Checkout"
        }
    };

    view! {
        <div>
            "Checkout Cancelled..."
        </div>
        // <div>
        //     "Checkout Session Id: "
        //     {move || {checkout_sessionid.get()}}
        // </div>
        // <Show
        //     when=move || {
        //         let stripe_data = match stripe_data.get()  {
        //             Some(ok) => ok,
        //             None => return false,
        //         }.unwrap();
        //         stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
        //     }
        //     fallback=move || view!{}
        // >
        //     <button on:click=move |_| {
        //         let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

        //         let mut url = String::new();

        //         if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
        //             url = session.url.to_owned().expect("Checkout session has no url!!!");
        //             spawn_local(async move {
        //                 redirect_to_url(url).await;
        //             })
        //         } else {
        //             leptos::logging::log!("No active checkout session.")
        //         }
        //     }>
        //         "Back to checkout session"
        //     </button>
        // </Show>

    }
}

#[component]
pub fn ShoppingCart() -> impl IntoView {
    let stripe_data = expect_context::<StripeDataRes>();
    provide_context(stripe_data);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    let set_shopping_cart = expect_context::<WriteSignal<ShoppingCart>>();
    provide_context(set_shopping_cart);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);
    let set_checkout_sessionid = expect_context::<WriteSignal<CheckoutSessionIdRes>>();
    provide_context(set_checkout_sessionid);

    let submit_checkout = expect_context::<ReadSignal<CheckoutSessionUpdateRes>>();
    provide_context(submit_checkout);
    let set_submit_checkout = expect_context::<WriteSignal<CheckoutSessionUpdateRes>>();
    provide_context(set_submit_checkout);

    let checkout_sessionid = expect_context::<Signal<CheckoutSessionIdRes>>();
    provide_context(checkout_sessionid);

    let checkout_session = expect_context::<CheckoutSessionRes>();
    provide_context(checkout_session);

    let checkout_loading = checkout_session.loading();
    let is_checkout_loading = move || {
        if checkout_loading() {
            "Loading..."
        } else {
            let session = checkout_session
                .get()
                .map(|value| {
                    value
                        .map(|value2| {
                            set_checkout_sessionid.update(|s| *s = value2.id.clone());
                            value2.id
                        })
                        .unwrap_or_else(|x| "Loading 2".into())
                })
                .unwrap_or_else(|| "Loading".into());
            // leptos::logging::log!("session: {:#?}", session);
            // stripe_data.get().expect("no stripdata lol")
            "Checkout"
        }
    };

    view! {
        <Show
            when=move || { shopping_cart.get().0.len() != 0 }
            fallback=|| view!{
                <div>
                    <h3>
                        "Your Shopping Cart is Empty."
                    </h3>
                    <p>
                        "You can browse items in:"
                    </p>
                    <a href="/shop/food">"Farm Food Shop "</a>
                    "or "
                    <a href="/shop/pet">"Pet Food Shop"</a>
                </div>
            }
        >
                <ul class="shopping-list-ul">
                    {
                        shopping_cart.get().0.into_iter()
                            .map(|(product_id, quantity)| {
                                let (product_id, _) = create_signal(product_id.clone());
                                provide_context(product_id);

                                let product_name = if let Some(product) = stripe_data
                                    .get()
                                    .unwrap()
                                    .unwrap()
                                    .products
                                    .into_iter()
                                    .find(|x| x.id == product_id.get())
                                {
                                    create_signal(product.name).0
                                } else {
                                    set_shopping_cart.update(|s| s.remove_single_product(&product_id.get()));
                                    create_signal("Product".to_string()).0
                                };

                                provide_context(product_name);
                                view! {
                                    <li>
                                        <p>
                                            {product_name.get()}", quantity: "{quantity}
                                        </p>
                                        <div>
                                            <Show
                                                when=move || {quantity < 20}
                                                fallback=move || view! {
                                                    <button class="plus_one_product_amount">
                                                    "MAX"
                                                    </button>
                                                }
                                            >
                                                <button class="plus_one_product_amount" on:click=move |_| {
                                                    set_shopping_cart.update(|s| {
                                                        s.add_single_product(&product_id.get(), 20);
                                                    });
                                                }>
                                                "+"
                                                </button>
                                            </Show>
                                            <button class="minus_one_product_amount" on:click=move |_| {
                                                set_shopping_cart.update(|s| {
                                                    s.remove_single_product(&product_id.get());
                                                });
                                            }>
                                                {move || if quantity > 1 {
                                                    "-"
                                                } else {
                                                    "Delete"
                                                }}
                                            </button>
                                        </div>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <div class="shopping-cart-ceckout-section">
                    <button class="checkout-button" on:click=move |_| {
                            let checkout_sessionid_before = checkout_sessionid.get();

                            set_submit_checkout.update(|s| {
                                *s += 1;
                            });

                            spawn_local(async {
                                stripe_sync().await;
                            });

                        }>
                        {is_checkout_loading} // checkout button text
                    </button>
                    <button on:click=move |_| {
                            set_shopping_cart.update(|s| {
                                *s = ShoppingCart::default();
                            });
                        }>
                        "Clear"
                    </button>
                    // <Show
                    //     when=move || {
                    //         let stripe_data = match stripe_data.get()  {
                    //             Some(ok) => ok,
                    //             None => return false,
                    //         }.unwrap();
                    //         stripe_data.checkout_sessions.iter().any(|session| session.id == checkout_sessionid.get())
                    //     }
                    //     fallback=move || view!{}
                    // >
                    //     <button on:click=move |_| {
                    //         let stripe_data = stripe_data.get().expect("No StripeData!").unwrap();

                    //         let mut url = String::new();

                    //         if let Some(session) = stripe_data.checkout_sessions.iter().find(|session| session.id == checkout_sessionid.get()) {
                    //             url = session.url.to_owned().expect("Checkout session has no url!!!");
                    //             spawn_local(async move {
                    //                 redirect_to_url(url).await;
                    //             })
                    //         } else {
                    //             leptos::logging::log!("No active checkout session.")
                    //         }
                    //     }>
                    //         "Back to checkout"
                    //     </button>
                    // </Show>
                </div>
        </Show>
    }
}

#[component]
pub fn FooterBar() -> impl IntoView {
    view! {
        <footer class="footerbar">
            <div class="footer-content">
                <div class="footer-section">
                    <p>"© 2024 FARMTASKER PTY LTD. All rights reserved."</p>
                    // <p>
                    //     "This website is licensed under the "
                    //     <a href="https://www.gnu.org/licenses/lgpl-2.1.html" target="_blank">"GNU Lesser General Public License v2.1"</a>.
                    // </p>
                </div>
                <div class="footer-section">
                    <p>
                        "Contact us: "
                        <a href="mailto:info@farmtasker.au">"info@farmtasker.au"</a>
                    </p>
                </div>
                // <div class="footer-section">
                //     <p>
                //         <a href="/privacy">"Privacy Policy"</a> |
                //         <a href="/terms">"Terms of Service"</a>
                //     </p>
                // </div>
            </div>
        </footer>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let selected = expect_context::<ReadSignal<CurrentPage>>();

    let (is_navbar_hidden, set_is_navbar_hidden) = create_signal(true);

    let shopping_cart = expect_context::<Signal<ShoppingCart>>();
    provide_context(shopping_cart);

    view! {
        <nav class="navbar">
            <div class="banner-bg">
                <div class="logo-container">
                    <a href="/" class="main_logo">
                        <img src="/navbar/shapka/webp/main_logo.webp" alt="Farmtasker Logo"/>
                    </a>
                    <button class="navbar-menu-button"
                        on:click=move |_| {
                            set_is_navbar_hidden.update(|n| *n = !*n);
                        }
                    >
                        <img src="/navbar/shapka/webp/menu_bars_tag.webp" alt="Menu"/>
                    </button>
                    <img src="/navbar/shapka/webp/farm_products_marketplace_tag.webp" class="navbar-welcome" alt="Welcome to farm products marketplace!"/>
                    <a href="/shop/cart" class="shopping-cart-button"
                        class:current=move || {
                            matches!(selected.get(), CurrentPage::ShoppingCart)
                        }
                    >
                        <img src="/navbar/shapka/webp/shopping_cart_tag.webp"/>
                        <div class="shopping-cart-counter">
                            {move || match shopping_cart.get().0.values().map(|&v| v as usize).sum() {
                                0 => "0".to_string(),
                                x => x.to_string(),
                            }}
                        </div>
                    </a>
                </div>
            </div>
            <ul class="nav_buttons"
                class:is-navbar-hidden=move || is_navbar_hidden()
                class:is-navbar-hidden-opposite=move || !is_navbar_hidden()
                >
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::HomePage)}
                        href="/" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Home"
                        />
                        <span class="overlay-text">Home</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::FarmFood)}
                        href="/shop/food" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Food Shop"
                        />
                        <span class="overlay-text">Farm Food</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::PetFood)}
                        href="/shop/pet" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Farm Pet Food"
                        />
                        <span class="overlay-text">Pet Food</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::EatNow)}
                        href="/shop/eat" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Eat Now"
                        />
                        <span class="overlay-text">Eat Now</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::VideoBlogs)}
                        href="/blog/culinary-adventure" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Video Blogs"
                        />
                        <span class="overlay-text">Video Blogs</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::Delivery)}
                        href="/delivery" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="Delivery"
                        />
                        <span class="overlay-text">Delivery</span>
                    </a>
                </li>
                <li>
                    <a
                        class:current=move || {matches!(selected.get(), CurrentPage::About)}
                        href="/about" id="button_middle"
                    >
                        <img
                             style="filter: brightness(1.2)"
                             src="/navbar/empty_button.png" class="button_middle_image" alt="About Us"
                        />
                        <span class="overlay-text">About Us</span>
                    </a>
                </li>
                // <li>
                //     <a
                //     class:current=move || {
                //         matches!(selected.get(), CurrentPage::ShoppingCart)
                //     }
                //         href="/shop/cart" id="button_middle">"🛒 ""Cart"</a>
                // </li>
            </ul>
        </nav>
    }
}
