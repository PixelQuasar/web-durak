export const Homepage = function () {
    return `
<div class="homepage-wrapper">
    Homepage
    <button class="demo-button">hello</button>
</div>
    `
}

document.querySelector("body").addEventListener("click", (event) => {
    switch (event.target.className) {
        case "demo-button": console.log("demo"); break;
        default: break;
    }
})
