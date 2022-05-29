document.addEventListener("DOMContentLoaded", () => {
    new MutationObserver((records, observer) => {
        for (const record of records) {
            for (const child of record.addedNodes) {
                if (child instanceof HTMLCanvasElement) {
                    document.body.querySelector("p")?.remove()
                    child.focus()
                    observer.disconnect()
                    return
                }
            }
        }
    }).observe(document.body, {
        childList: true,
    })
})
