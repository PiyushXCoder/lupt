let Camera = class {
    static setupCamera() {
        $('#file-input')[0].addEventListener('change', function(e) {
            const file = e.target.files[0];
            if (!file) {
                return;
            }
            Camera.compressImage(file, 0.1, 'image/webp');
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
                    if(base64data.length > 63488 && mime != 'image/jpeg') {
                        base64data = null;
                        result = null;
                        Camera.compressImage(file, 0.7, 'image/jpeg');
                        return;
                    } else if(base64data.length > 63488 && mime == 'image/jpeg') {
                        window.alert('file is too large!');
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