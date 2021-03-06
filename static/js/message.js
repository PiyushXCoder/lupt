
let Messages = class {
    static pick(elm) {
        var elm = $(elm);
        elm.toggleClass('active');
        autosize($('#send_box')[0]);
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
            if(e.key === 'Enter' && !e.shiftKey) {
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
            text += name+'('+val+')'+ ','
        })
        text = text.substr(0, text.length-1);
        text += ' is typing...'
        $('#status_area').append($('<div>', { id: 'typing' }).append(text));

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static pushMessage(sender, appendElm, msg_id) {
        var isMe = myinfo.kunjika == sender;
        var area = $('#message_area');
        var parent = $('<div>');
        var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other'), msgid: msg_id});
        elm.append($('<div>', {class: 'message-sub', name: 'by'})
            .append($('<span>').text(vayakti[sender]+'('+sender+')'))
            .append($('<span>', {class: 'pull-right'}).text(Messages.currentTime())));
        appendElm.forEach(function(app) {
            elm.append(app);
        });
        elm.click(function() {
            Messages.pick(this);
        });
        elm.on('taphold', { delay: 700 },function () {
            if(parent.find('#react_bar').length > 0) {
                $('#react_bar').remove();
                return
            } else if($('#react_bar').length > 0) $('#react_bar').remove();
            var ee = $('<div>', {id: 'react_bar'});
            ['like', 'heart', 'laugh', 'sad'].forEach(function(i) {
                ee.append($('<button>', {class: 'button', style: 'padding: 1rem'})
                .append($('<img>', {src: 'img/'+i+'.svg', height: '18'}))
                .click(function(evt) {
                    sendReaction(i,msg_id);
                }));
            })
            ee.append($('<button>', {class: 'button', style: 'padding: 1rem'})
                .append($('<img>', {src: 'img/close.svg', height: '18'}))
                .click(function(evt) {
                    $('#react_bar').remove();
                }));
            parent.append(ee);
        });
        parent.append(elm);
        area.append(parent);

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static pushText(sender, text, reply = null, msg_id) {
        var app = [];
        if(reply != null && reply.length > 0) {
            app.push(
                $('<div>', {class: 'message message-reply'})
                .append($('<pre>').text(reply))
            );
        }
        app.push($('<pre>').text(text));
        Messages.pushMessage(sender,app,msg_id);
    }

    static pushImage(sender, src, msg_id) {
        var sp = $('<span>', {class: 'text-grey bd-light'}).append(' Loading Image... ');
        var img = $('<img>', {src: src, width: 300});
        img.on('load', function() {
            sp.empty();
            sp.append(img);
        });
        Messages.pushMessage(sender,[sp],msg_id);
    }

    static addReaction(sender, emoji, msg_id) {
        var msg = $('[msgid='+msg_id+']');
        if(msg.find('[name=bar_msg]').length == 0) {
            msg.append($('<div>', {class: 'message-sub', name: 'bar_msg'}));
        }
        var bar = msg.find('[name=bar_msg]');
        var elm = bar.find('[name="r_'+sender+'"]');
        if(elm.length > 0) {
            elm.find('img').attr('src', 'img/'+emoji+'.svg');
            elm.find('span').text(vayakti[sender]+'('+sender+')');
        } else {
            var elm = $('<span>', {name: 'r_'+sender, style: 'padding: 0px 4px; display: inline-flex'});
            elm.append($('<img>', {style: 'height: 1.4rem', src: 'img/'+emoji+'.svg'}));
            elm.append($('<span>').text(vayakti[sender]+'('+sender+')'));
            bar.append(elm);
        }
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
    
    static prepareReply() {
        var text = "";
        $('.message.active').each(function() {
            text += $(this).find('[name=by]').find('span:not(.pull-right)').text() + ' : ';
            text += $(this).find('pre').last().text() + '\n';
        });
        var el = $('#reply_clip');
        el.removeClass('is-hidden');
        el.attr('msg', text);
        $('#reply_clip > span').text(text.substr(0,28)+ '...');
        Messages.unselectAll();
    }
    
    static copyMessagesToClipboard() {
        var text = "";
        $('.message.active').each(function() {
            text += $(this).find('[name=by]').find('span:not(.pull-right)').text() + ' : ';
            var pr = $(this).find('pre');
            if (pr.length > 1) text += '\n';
            for(var i = 0; i < pr.length-1; i++) {
                $(pr[i]).text().split('\n').forEach(function (t) {
                    if(t.trim().length <= 0) return;
                    text += '    ' + t + '\n';
                });
            }
            text += pr.last().text();
        });

        var $temp = $("<textarea>");
        $("body").append($temp);
        $temp.val(text).select();
        document.execCommand("copy");
        $temp.remove();
        Messages.unselectAll();
    }


    static prepareEditMessages() {
        var msgs = $('.message.active');
        if(msgs.length > 1) {
            var prop = {
                title: 'Warning',
                text: 'Select only one Message!',
                check: false
            }; dialog(prop, function() {});
            return;
        }

        var msg = $(msgs[0]);
        if(msg.find('pre').length == 0 || !msg.hasClass('message-me')) {
            var prop = {
                title: 'Warning',
                text: 'Can\'t edit this Message!',
                check: false
            }; dialog(prop, function() {});
            return;
        }

        var prop = {
            title: 'Edit Messages',
            text: 'Do you really want to edit?',
            check: false
        }

        dialog(prop, function() {
            var msg = $($('.message.active')[0]);
            $('#reply_clip > span').text(msg.find('pre').text().substr(0, 20)+ '...');
            $('#reply_clip').attr('msg', msg.attr('msgid'));
            Messages.unselectAll();
            $('#replyicon').removeClass('is-hidden');
            $('#reply_clip').removeClass('is-hidden');
        });

        $('#selected_clip').addClass('is-hidden');
    }
    
    static cleanMessage() {
        $('#message_area').empty();
        $('#status_area').empty();
        $('#action_clip').addClass('is-hidden');
    }

    static deleteMessages(msgid) {
        msgid.forEach(function(id) {
            $('[msgid='+id+']').remove();
        });
    }

    static editMessages(msgid, text) {
        var msg = $('[msgid='+msgid+']');
        if(msg.find('[name=edited]').length == 0) {
            msg.find('[name=by]').append($('<span>', {name: 'edited', class: 'text-grey bd-light', style: 'margin-left: 4px; padding: 0px 3px'}).text('edited'));
        }
        $('[msgid='+msgid+']').find('pre').last().text(text);
    }

    static currentTime() {
        var today = new Date();
        return today.getHours()+':'+('0' + today.getMinutes()).slice(-2);
    }
}
