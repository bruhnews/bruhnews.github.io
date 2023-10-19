setTimeout(show, 1000);
let modalHidden = false;
let articleClicked = false; // Started

// Show cookie modal
function show() {
    if (document.querySelector(".modal-backdrop")) {
        document.querySelector(".modal-backdrop").setAttribute("show", true);
    }
    if (document.querySelector("main")) {
        document.querySelector("main").setAttribute("blur", true);
    }
}

// Hide cookie modal
function hide() {
    if (document.querySelector(".modal-backdrop")) {
        document.querySelector(".modal-backdrop").removeAttribute("show");
    }
    document.querySelector("main").removeAttribute("blur");
    modalHidden = true;
    start();
}

// Navigate to article page
function clickArticle() {
    if (!modalHidden) {
        return;
    }

    setTimeout(() => {
        alert(
            "An error occurred: Blocked content\nPlease contact system administrator",
        );
    }, (Math.random() + 0.5) * 1500);
}

// Start process
function start() {
    // Unfocus all buttons
    document.querySelectorAll(".button").forEach(e => {
        e.blur();
    });

    // Prevent starting
    if (articleClicked || navigator.userAgent === "bruhnews") {
        console.log("Your two smart!");
        return;
    }
    articleClicked = true;

    // Show process
    document.querySelector("#image").style.visibility = "hidden";
    document.querySelector("#video").currentTime = 0.3;
    document.querySelector("#video").play();
    document.querySelector(".video").style.display = "block";

    // Console
    console.warn(
        "%cYou have been fooled!.\nSend this to your friends to totally fool them too!",
        "font-size:20px",
    );
    console.warn("%cBruhNews!", "font-size:20px");
    console.log("Made by darcy");

    // Show image
    setTimeout(() => {
        document.querySelector("#image").style.visibility = "visible";
    }, 2000);
    return;
}

// Exit process
function exit() {
    document.querySelector("#video").pause();
    document.querySelector(".video").style.display = "none";
}

// Prevent leaving
window.onbeforeunload = function () {
    if (articleClicked) {
        return "Don't you want to stay a little longer?";
    }
};

// Add event listeners
setTimeout(() => {
    ["keypress", "mousedown", "click"].forEach(name => {
        window.addEventListener(name, () => {
            if (articleClicked) {
                return;
            }
            hide();
        });
    });
}, 100);
