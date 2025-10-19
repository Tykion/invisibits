import init, { wav_decode_wrapper } from "../pkg/stego_wasm.js"

async function main() {

    await init(); // initialize wasm module

    const wavInput = document.getElementById("wav-decode")
    const passwordInput = document.getElementById("password-decode")
    const decodeButton = document.getElementById("decode-button")
    const errorMessage = document.getElementById("error-message")
    const messageOutput = document.getElementById("decoded-message")


    let downloadButton = document.createElement("a");
    downloadButton.textContent = "Download Message"
    downloadButton.style.display = "none"
    downloadButton.className = "button"
    downloadButton.style.marginTop = "10px"
    document.querySelector(".container-lg").appendChild(downloadButton);

    decodeButton.addEventListener("click", async () => {
        errorMessage.textContent = ""
        messageOutput.value = ""
        downloadButton.style.display = "none"

        const file = wavInput.files[0]
        const password = passwordInput.value || null

        if (!file) {
            errorMessage.textContent = "Please select a WAV file!"
            return
        }

        if (!["audio/wav"].includes(file.type)) {
            errorMessage.textContent = "Supported type is WAV"
            return
        }

        try {
            const arrayBuffer = await file.arrayBuffer()
            const bytes = new Uint8Array(arrayBuffer)

            const decodedMessage = wav_decode_wrapper(bytes, password)

            messageOutput.value = decodedMessage

            if (decodedMessage == "") {
                errorMessage.textContent = "No message here"
            }

            downloadButton.onclick = () => {
                const blob = new Blob([decodedMessage], { type: "text/plain" })
                const url = URL.createObjectURL(blob)
                const a = document.createElement("a")
                a.href = url
                a.download = `${file.name.split(".")[0]}_message.txt`
                document.body.appendChild(a)
                a.click()
                a.remove()
                URL.revokeObjectURL(url)
            }

            downloadButton.style.display = "inline-block"
        } catch (err) {
            console.error(err)
            if (err)
                errorMessage.textContent = err
            
        }
    })

}

main()