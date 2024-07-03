export const ErrorPage = function () {
    const props = {
        errorName: this.errorName ?? "Unknown error name",
        errorStack: this.errorStack ?? "Unknown error stack"
    }
    return `
<div>
    <h1>${props.errorName}</h1>
    ${props.errorStack.split("\n").map(item => `<div>${item}</div>`).join("")}
</div>
`
}
