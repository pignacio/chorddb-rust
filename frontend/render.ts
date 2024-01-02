export interface RenderBit {
    html: HTMLElement,
    size: number,
    position: number,
}


export function renderInSingleLine(bits: RenderBit[]): HTMLElement {
    console.log("Rendering bits: ", bits)
    let html = createSpan("line")
    bits.sort((a, b) => a.position - b.position)

    let currentPosition = 0;
    for (let bit of bits) {
        while (currentPosition < bit.position) {
            html.innerHTML += " "
            currentPosition++
        }
        html.appendChild(bit.html)
        currentPosition += bit.size
    }
    return html;
}


export function createSpan(cls: string, inner?: string) {
    let span = document.createElement("span")
    span.classList.add(cls);
    if (inner != undefined) {
        span.innerHTML = inner
    }
    return span
}