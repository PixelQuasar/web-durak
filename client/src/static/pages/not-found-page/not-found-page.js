export const NotFoundPage = function () {
    const props = {
        notFoundPage: this.notFoundPage ?? "unknown",
    }
    return `
<div>
    <h1>
        "${props.notFoundPage}" instance is not found.
    </h1>
</div>
    `
}
