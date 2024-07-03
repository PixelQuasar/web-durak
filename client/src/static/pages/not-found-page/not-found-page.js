export const NotFoundPage = function () {
    const props = {
        notFoundPage: this.notFoundPage ?? "unknown",
    }
    return `
<div>
    <h1>
        "${props.notFoundPage}" page not found.
    </h1>
</div>
    `
}
