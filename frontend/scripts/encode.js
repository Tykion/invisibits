import init, { encode_wrapper } from "../pkg/stego_wasm.js"

async function main() {

    await init(); // initialize wasm module

    const imageInput = document.getElementById("image-encode")
    const messageInput = document.getElementById("message-encode")
    const passwordInput = document.getElementById("password-encode")
    const encodeButton = document.getElementById("encode-button")
    const errorMessage = document.getElementById("error-message")

    let outputImg = document.createElement("img")
    outputImg.style.display = "block"
    outputImg.style.marginTop = "10px"
    document.querySelector(".container-lg").appendChild(outputImg);

    let downloadButton = document.createElement("a");
    downloadButton.textContent = "Download Encoded Image"
    downloadButton.style.display = "none"
    downloadButton.className = "button"
    downloadButton.style.marginTop = "10px"
    document.querySelector(".container-lg").appendChild(downloadButton);

    encodeButton.addEventListener("click", async () => {
        errorMessage.textContent = ""
        downloadButton.style.display = "none"

        const file = imageInput.files[0]
        const message = messageInput.value
        const password = passwordInput.value || null

        const hasMoreThanAscii2 = !/^[\u0000-\u007f]*$/.test(message)

        if (hasMoreThanAscii2 == true) {
            errorMessage.textContent = "Please only use valid characters"
            return
        }

        if (!file) {
            errorMessage.textContent = "Please select an image!"
            return
        }

        if (!["image/png", "image/jpeg"].includes(file.type)) {
            errorMessage.textContent = "Supported types are PNG and JPEG"
            return
        }

        if (file.size > 5 * 1024 * 1024) {
            errorMessage.textContent = "Image size can't be more than 5 MB"
            return
        }

        try {
            const arrayBuffer = await file.arrayBuffer()
            const bytes = new Uint8Array(arrayBuffer)

            const encodedBytes = encode_wrapper(message, bytes, password)

            const blob = new Blob([encodedBytes], { type: "image/png "})
            const url = URL.createObjectURL(blob)

            const originalName = file.name.split(".")[0]

            outputImg.src = url

            downloadButton.href = url
            downloadButton.download = `${originalName}_steg.png`
            downloadButton.style.display = "inline-block"

        } catch (err) {
            console.error(err)
            
            if (err)
                errorMessage.textContent = err
            
        }
    })

}

main()