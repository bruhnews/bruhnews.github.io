use ibex::prelude::*;
use ibex::{routes, ssg};

use bruhnews::{ARTICLES, BEST_HEADLINES};

const URL_ROOT: &str = "https://bruhnews.github.io/";

const REPO_URL: &str = "https://github.com/bruhnews/bruhnews.github.io";
const SALE_URL: &str = "#"; //TODO

fn main() {
    let routes = routes! [
        (/)       => at_index(),
        (/404)    => at_404(),
        (/"help") => at_help(),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_index() -> Document {
    document! { [lang="en"]
        @homepage[false]
    }
}
fn at_help() -> Document {
    document! { [lang="en"]
        @homepage[true]
        script { "start();" }
    }
}

fn homepage(mute_video: bool) -> View {
    view! { @use_basic {
        HEAD {
            script { [include_str!("js/script.js")] }
        }

        main {
            h1 ."header" { "BruhNews - The Strongest News" }
            @list []
        }

        div ."modal-backdrop" {
            div ."modal" {
                h1 { "Notice" }
                p {
                    "BruhNews uses cookies to improve your experience."
                    ~ "By using our website, you agree to cookies"
                }

                button ."big" [onclick="hide()"] { "Accept Cookies" }
                br/
                button [onclick="hide()"] { "Disable All Cookies" }
            }
        }

        div ."video" {
            video #"video" [loop!, preload="auto", muted?=mute_video] {
                source [src=url!("static/video.mp4"), type="video/mp4"]/
            }
            script { "document.querySelector(\"#video\").load();" }
            img #"image" [
                alt="Overlay image",
                src=url!("static/image.png"),
            ]/
        }
    } }
}

fn list() -> View {
    view! {
        ul ."list" {
            [:for (i, article) in ARTICLES.into_iter().enumerate() {
                li ."article" {
                    a [href="#"] {
                        img [
                            alt=article.title,
                            src=url!(format!("static/thumbs/{}.jpg", i)),
                            height=50,
                        ]/
                        div ."text" {
                            div ."wrap" {
                                h1 ."headline" { [article.headline] }
                                p {
                                    span ."title" { [article.title] ":" ~ }
                                    span ."description" { [article.description] }
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
    document! { [lang="en"]
        @use_basic {
        main {
            h1 ."header" { "BruhNews - 404" }

            h2 {
                a [href=url!()] { ">> Back to home page <<" }
            }
        }
    } }
}

fn use_basic(children: View) -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .desc(format!( "Subscribe to bruh.news for daily updates\nLatest: {}", BEST_HEADLINES.join(", ")))
                .image(url!("static/icon.png"))
                .color("#D80000")
                .author("darcy")
            ]
            meta [name="google-site-verification", content="Z4K-H95bEts5-JJ9GOuWUDSz5fhLKvlg4qu5_KhLez8"]/

            title { "BruhNews - The Strongest News" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/main.css")]/

            script [async!, src="https://www.googletagmanager.com/gtag/js?id=G-8Y8DXGJ04S"]/
            script {r#"
                window.dataLayer = window.dataLayer || [];
                function gtag() {
                    dataLayer.push(arguments);
                }
                gtag("js", new Date());
                gtag("config", "G-8Y8DXGJ04S");
            "#}

            @ssg::use_autoreload []
        }

        [children]

        footer {
            a [href=REPO_URL, target="_blank"] { "Learn more" }
            a [href=SALE_URL, target="_blank"] { "Buy this domain" }
        }
    }
}
