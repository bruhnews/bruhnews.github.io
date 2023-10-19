use ibex::prelude::*;
use ibex::{routes, ssg};

use bruhnews::ARTICLES;

const URL_ROOT: &str = "/ibex-template/";

const REPO_URL: &str = "https://github.com/bruhnews/bruhnews.github.io";
const SALE_URL: &str = "#"; //TODO

fn main() {
    let routes = routes! [
        (/)       => at_index(),
        (/404)    => at_404(),
        (/"help") => at_help(),
    ];

    ssg::quick_build(routes).unwrap();
    println!("All done!");
}

fn at_index() -> Document {
    view! {
        @homepage[false]
    }
    .into()
}
fn at_help() -> Document {
    view! {
        @homepage[true]
        script { "start();" }
    }
    .into()
}

fn homepage(mute_video: bool) -> View {
    view! { @use_basic {
        HEAD {
            script { [include_str!("js/script.js")] }
        }

        main {
            h1 [class="header"] { "BruhNews - The Strongest News" }
            @list
        }

        div [class="modal-backdrop"] {
            div [class="modal"] {
                h1 { "Notice" }
                p {
                    "BruhNews uses cookies to improve your experience."
                    ~ "By using our website, you agree to cookies"
                }

                button [class="big", onclick="hide()"] { "Accept Cookies" }
                br/
                button [onclick="hide()"] { "Disable All Cookies" }
            }
        }

        section [class="video"] {
            video [loop=true, id="video", preload="auto", muted?=mute_video] {
                source [src=url!("static/video.mp4"), type="video/mp4"]/
            }
            script { r##"document.querySelector("#video").load();"## }
            img [src=url!("static/image.png"), id="image"]/
        }
    } }
}

fn list() -> View {
    view! {
        ul [class="list"] {
            [:for (i, article) in (ARTICLES.into_iter().enumerate()) {
                li [class="article"] {
                    a [href="#"] {
                        img [
                            alt=article.title,
                            src=url!(format!("static/thumbs/{}.jpg", i)),
                            height=50,
                        ]/
                        div [class="text"] {
                            div [class="wrap"] {
                                h1 [class="headline"] { [article.headline] }
                                p {
                                    span [class="title"] { [article.title] ":" ~ }
                                    span [class="description"] { [article.description] }
                                }
                            }
                        }
                    }
                }
            }]
        }
    }
}

fn at_404() -> Document {
    view! { @use_basic {
        main {
            h1 [class="header"] { "BruhNews - 404" }

            h2 {
                a [href=url!()] { ">> Back to home page <<" }
            }
        }
    } }
    .into()
}

fn use_basic(children: View) -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .desc("Subscribe to bruh.news for daily updates")
                .image(url!("static/icon.png"))
                .color("#D80000")
                .author("darcy")
            ]
            meta [name="google-site-verification", content="Z4K-H95bEts5-JJ9GOuWUDSz5fhLKvlg4qu5_KhLez8"]/

            title { "BruhNews - The Strongest News" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/main.css")]/

            script [async="true", src="https://www.googletagmanager.com/gtag/js?id=G-8Y8DXGJ04S"]/
            script {r#"
                window.dataLayer = window.dataLayer || [];
                function gtag() {
                    dataLayer.push(arguments);
                }
                gtag("js", new Date());
                gtag("config", "G-8Y8DXGJ04S");
            "#}
        }

        [children]

        footer {
            a [href=REPO_URL, target="_blank"] { "Learn more" }
            a [href=SALE_URL, target="_blank"] { "Buy this domain" }
        }
    }
}
