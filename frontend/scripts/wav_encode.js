import init, { wav_encode_wrapper } from "../pkg/stego_wasm.js"

async function main() {

    await init(); // initialize wasm module

    const wavInput = document.getElementById("wav-encode")
    const messageInput = document.getElementById("message-encode")
    const passwordInput = document.getElementById("password-encode")
    const encodeButton = document.getElementById("encode-button")
    const errorMessage = document.getElementById("error-message")

    let outputWav = document.createElement("audio")
    outputWav.controls = false
    document.querySelector(".container-lg").appendChild(outputWav);

    let downloadButton = document.createElement("a");
    downloadButton.textContent = "Download Encoded WAV"
    downloadButton.style.display = "none"
    downloadButton.className = "button"
    downloadButton.style.marginTop = "10px"
    document.querySelector(".container-lg").appendChild(downloadButton);

    encodeButton.addEventListener("click", async () => {
        errorMessage.textContent = ""
        downloadButton.style.display = "none"

        const file = wavInput.files[0]
        const message = messageInput.value
        const password = passwordInput.value || null

        const hasMoreThanAscii2 = !/^[\u0000-\u007f]*$/.test(message)

        if (hasMoreThanAscii2 == true) {
            errorMessage.textContent = "Please only use valid characters"
            return
        }

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

            const encodedBytes = wav_encode_wrapper(message, bytes, password)

            const blob = new Blob([encodedBytes], { type: "audio/wav"})
            const url = URL.createObjectURL(blob)

            const originalName = file.name.split(".")[0]

            outputWav.src = url

            downloadButton.href = url
            downloadButton.download = `${originalName}_steg.wav`
            downloadButton.style.display = "inline-block"
            outputWav.controls = true


        } catch (err) {
            console.error(err)
            
            if (err)
                errorMessage.textContent = err
            
        }
    })

}

main()