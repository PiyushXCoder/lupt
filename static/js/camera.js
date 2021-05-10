var Camera = class {
    static setupCamera() {
        $('#file-input')[0].addEventListener('change', (e) => {
            const file = e.target.files[0];
            if (!file) {
                return;
            }
            this.compressImage(file, 0.1, 'image/webp');
        });
    }    

    static compressImage(file, qual, mime) {
        new Compressor(file, {
            quality: qual,
            width: 320,
            mimeType: mime,
            success(result) {
                var reader = new FileReader();
                reader.readAsDataURL(result); 
                reader.onloadend = function() {
                    var base64data = reader.result;
                    if(base64data.length > 63000 && mime != 'image/jpeg') {
                        base64data = null;
                        result = null;
                        Camera.compressImage(file, 0.7, 'image/jpeg');
                        return;
                    }
                    
                    socket.send(JSON.stringify({
                        cmd: "img",
                        src: base64data
                    }));
                    $('#action_clip').addClass('is-hidden');
                }
            },
        });
    }
}