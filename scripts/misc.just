


#>> Local audio conversion using LAME
@convert-local:
    #!/bin/bash
    SOURCE_DIR="/Users/USER/Music/audify"
    OUTPUT_DIR="$SOURCE_DIR"

    for wav_file in "$SOURCE_DIR"/*.wav; do
        base_name=$(basename "$wav_file" .wav)
        mp3_file="$OUTPUT_DIR/$base_name.mp3"
        lame "$wav_file" "$mp3_file"
    done

    rm "$SOURCE_DIR"/*.wav

[working-directory: 'lib']
download-models:
    wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx     
    wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json

sign-android: keytool -genkey -v -keystore ~/upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload