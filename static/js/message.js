
let Messages = class {
    static pick(elm) {
        var elm = $(elm);
        elm.toggleClass('active');

        if($('.message.active').length == 0)
            $('#selected_clip').addClass('is-hidden');
        else
            $('#selected_clip').removeClass('is-hidden');
    }

    static unselectAll(msg) {
        $('.active').each(function() {
            $(this).removeClass('active');
        });
        $('#selected_clip').addClass('is-hidden');
    }

    static setupTyping() {
        var send_typing = false;
        var timeout = null;
        $('#send_box').keydown(function(e) {
            if (!send_typing) {
                sendTyping();
                send_typing = true;
                return;
            }
            clearTimeout(timeout);
            timeout = setTimeout(function() {
                send_typing = false;
                sendTypingEnd();
            },2000);
        });
        $('#send_box').keypress(function(e) {
            if(e.originalEvent.charCode == 13 && !e.shiftKey) {
                send();
                e.preventDefault();
                clearTimeout(timeout);
                send_typing = false;
                sendTypingEnd()
                return
            }
        });
    }

    static pushTypingStatus() {
        var elm = $('#status_area > #typing');
        if(elm.length > 0) elm.remove();
        if(typing.length == 0) return;
        var text = '';
        typing.forEach(function(val) {
            var name = vayakti[val];
            if(name == undefined) name = "";
            text += name+'('+val.substr(0,8)+')'+ ','
        })
        text = text.substr(0, text.length-1);
        text += ' is typing...'
        $('#status_area').append($('<div>', { id: 'typing' }).append(text));

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static pushMessage(sender, text, reply = null, msg_id) {
        var isMe = myinfo.kunjika == sender;
        var area = $('#message_area');
        var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other'), msgid: msg_id});
        if(!no_name_message) {
            if(sender == myinfo.kunjika)
                elm.append($('<div>', {class: 'message-by'}).append('me'))
            else
                elm.append($('<div>', {class: 'message-by'}).append(vayakti[sender]+'('+sender.substr(0, 8)+')'))
        } 
        if(reply != null && reply.length > 0) {
            elm.append(
                $('<div>', {class: 'message message-reply'})
                .append($('<pre>').append(reply))
            );
        }
        elm.append($('<pre>').append(text));
        elm.click(function() {
            Messages.pick(this);
        });
        area.append(elm);

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static pushImage(sender, src, msg_id) {
        var isMe = myinfo.kunjika == sender;
        var area = $('#message_area');
        var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other'), msgid: msg_id});
        if(!no_name_message) {
            if(sender == myinfo.kunjika)
                elm.append($('<div>', {class: 'message-by'}).append('me'))
            else
                elm.append($('<div>', {class: 'message-by'}).append(vayakti[sender]+'('+sender.substr(0, 8)+')'))
        } 
        elm.append($('<img>', {src: src, width: 300}));
        elm.click(function() {
            Messages.pick(this);
        });
        area.append(elm);

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    // in message area 
    static pushStatus(text) {
        var area = $('#message_area');
        var elm = $('<div>', {class: 'status'});
        elm.append($('<small>', {class: 'tag'}).append(text));
        area.append(elm);

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static selectedMessageToText() {
        var text = "";
        $('.active').each(function() {
            $(this).find('pre').each(function() {
                text += $(this).text() + '\n' 
            });
        });
    
        return text.trim();
    }
    
    static prepareReply() {
        var text = this.selectedMessageToText();
        var el = $('#reply_clip');
        el.removeClass('is-hidden');
        el.attr('msg', text);
        $('#reply_clip > span').text(text.substr(0, 15)+ '...');
        Messages.unselectAll();
    }
    
    static copyMessagesToClipboard() {
        var $temp = $("<textarea>");
        $("body").append($temp);
        $temp.val(this.selectedMessageToText()).select();
        document.execCommand("copy");
        $temp.remove();
        Messages.unselectAll();
    }
    
    static cleanMessage() {
        $('#message_area').empty();
        $('#status_area').empty();
        $('#action_clip').addClass('is-hidden');
    }

    static currentTime() {
        var today = new Date();
        return today.getHours()+':'+('0' + today.getMinutes()).slice(-2);
    }
}
