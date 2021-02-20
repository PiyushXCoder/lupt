$(document).ready(() => {
    $(".siimple-tabs-item").click(function() {
        var t = $(this);
        var tab = $(t.parents('tabs').first());
        tab.find('form').each((i,elm) => {
            var elm = $(elm);
            if(elm.attr('name') == t.attr('for')) 
                elm.removeClass('hidden');
            else
                elm.addClass('hidden');
        });

        t.parent().find('.siimple-tabs-item').each((i, elm) => {
            var elm = $(elm);
            if(elm.attr('for') === t.attr('for'))
                elm.addClass('siimple-tabs-tab--selected');
            else
                elm.removeClass('siimple-tabs-tab--selected');
        });
    });

    $('.message-me, .message-other').click(function() {
        selectMessage(this);
    });

    $('#selected_clip > .siimple-close').click(function() {
        unselectMessages();
    });

    var send_typing = false;
    var timeout = null;
    $('#send_box').keypress(function(e) {
        if(e.originalEvent.charCode == 13 && !e.shiftKey) {
            send();
            e.preventDefault();
            clearTimeout(timeout);
            send_typing = false;
            sendTypingEnd()
            return
        }
        if (!send_typing) {
            sendTyping();
            send_typing = true;
            return;
        }
        clearTimeout(timeout);
        timeout = setTimeout(function() {
            send_typing = false;
            sendTypingEnd();
        },3000);
    });

    $('#send_box').bind('input propertychange keyup', function() {
        var height = ($(window).height()*0.0165).toFixed(0)*20;
        var sheight = this.scrollHeight;
        if(sheight < height) {
            $(this).height(0);
            height = this.scrollHeight;
            $(this).height(height - 20);
            $('#reply_clip').css('bottom',  (this.scrollHeight + 10) + 'px');
            $('#selected_clip').css('bottom',  (this.scrollHeight + 10) + 'px');
        }
    });
});

function sendTyping() {
    socket.send(JSON.stringify({
        cmd: 'status',
        status: "typing"
    }));
}

function sendTypingEnd() {
    socket.send(JSON.stringify({
        cmd: 'status',
        status: "typing_end"
    }));
}

function calcHeight(value) {
    let numberOfLineBreaks = (value.match(/\n/g) || []).length;
    // min-height + lines x line-height + padding + border
    let newHeight = 20 + numberOfLineBreaks * 20 + 12 + 2;
    return newHeight;
}

// Create WebSocket connection.
const socket = new WebSocket('ws://'+window.location.host+'/ws/');
var callbacks = [];
var myinfo = {
    kunjika: "",
    name: ""
};
var vayaktiList = {};
var typing = [];
var no_name_message = false;

// Connection opened
socket.addEventListener('open', function (event) {
    $('#progressbar').toggleClass('hidden');
});

// Listen for messages
socket.addEventListener('message', function (event) {
    var j = JSON.parse(event.data);
    switch(j.cmd) {
        case 'resp':
            if(j.result == 'Err') {
                if($('#chat_panel').hasClass('hidden')) {
                    $('[name="error_msg"]').text(j.message);
                    $('[name="error_msg"]').removeClass('hidden');
                    $('#progressbar').addClass('hidden');
                    callbacks = [];
                } else {
                    pushStatus(j.message);
                }
            } else if(j.result == 'Ok'){
                if(callbacks.length > 0) {
                    callbacks[0]();
                    callbacks.shift();
                }
            }
            break;
        case 'random':
            callbacks[0]();
            callbacks = [];
            no_name_message = true;
            $('#next_btn').removeClass('hidden');
            pushStatus('Say hi to '+j.name);
            vayaktiList[j.kunjika] = j.name;
            break;
        case 'status':
            if(j.status == "typing") {
                typing.push(j.kunjika);
                pushTypingStatus();
            } else if(j.status == "typing_end") {
                const index = typing.indexOf(j.kunjika);
                if (index > -1) typing.splice(index, 1);
                pushTypingStatus();
            }
            break;
        case 'text':
            pushMessage(j.kunjika, j.text, j.reply);
            break;
        case 'connected':
            vayaktiList[j.kunjika] = j.name;
            pushStatus('Vyakti '+j.name+' connected as '+j.kunjika);
            break;
        case 'disconnected':
            delete vayaktiList[j.kunjika];
            pushStatus('Vyakti '+j.name+' disconnected as '+j.kunjika);
            break;
        case 'list':
            JSON.parse(j.vayakti).forEach(function(usr) {
                vayaktiList[usr[0]] = usr[1];
            });
            break;
    }
});

var joining = false;
function connect(frm) {
    if(joining) return;
    joining = true;
    var frm = $(frm);
    $('#progressbar').removeClass('hidden');
    var data = {};
    frm.serializeArray().forEach(el => {
        data[el.name] = el.value;
    });
    if(data['length'] !== undefined) {
        data['length'] = parseInt(data['length']);
    }
    callbacks.push(() => {
        cleanMessage();
        $('#progressbar').addClass('hidden');
        $('#send_box').text('');
        $('#connect_panel').addClass('hidden');
        $('[name="error_msg"]').addClass('hidden');
        $('#chat_panel').removeClass('hidden');
        $('#send_box').focus();
        $('#next_btn').addClass('hidden');
        myinfo.kunjika = data.kunjika;
        myinfo.name = data.name;
        no_name_message = false;
        joining = false;
        socket.send(JSON.stringify({cmd: 'list'}));
    });
    socket.send(JSON.stringify(Object.assign({cmd: frm.attr('cmd')}, data)));
}

function connect_next() {
    if(joining) return;
    joining = true;
    $('#progressbar').removeClass('hidden');
    callbacks.push(() => {
        cleanMessage();
        $('#progressbar').addClass('hidden');
        $('#send_box').text('');
        $('#connect_panel').addClass('hidden');
        $('[name="error_msg"]').addClass('hidden');
        $('#chat_panel').removeClass('hidden');
        $('#send_box').focus();
        $('#next_btn').addClass('hidden');
        joining = false;
        socket.send(JSON.stringify({cmd: 'list'}));
    });
    socket.send(JSON.stringify({ cmd: 'randnext' }));
}

function leave() {
    callbacks.push(() => {
        $('#chat_panel').addClass('hidden');
        $('#reply_clip').addClass('hidden');
        $('#selected_clip').addClass('hidden');
        $('#action_clip').addClass('hidden');
        $('#connect_panel').removeClass('hidden');
        myinfo.kunjika = '';
        myinfo.name = '';
    });
    socket.send(JSON.stringify({
        cmd: 'leave'
    }));
}

function pushTypingStatus() {
    var elm = $('#status_area > #typing');
    if(elm.length > 0) elm.remove();
    if(typing.length == 0) return;
    var text = '';
    typing.forEach((val)  => {
        text += val + ','
    })
    text = text.substr(0, text.length-1);
    text += ' is typing...'
    $('#status_area').append($('<div>', { id: 'typing', 
        class:'siimple-label siimple--mx-2 siimple--my-0' }).append(text));

    var scroll = $("#message_area_scroll");
    scroll.scrollTop(scroll[0].scrollHeight);
}

function pushMessage(sender, text, reply = null) {
    var isMe = myinfo.kunjika == sender;
    var area = $('#message_area');
    var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other')});
    if(!no_name_message) {
        if(sender == myinfo.kunjika)
            elm.append($('<div>', {class: 'message-by'}).append('me'))
        else
            elm.append($('<div>', {class: 'message-by'}).append(vayaktiList[sender]+'('+sender+')'))
    } else {
        elm.addClass('siimple--py-1');
    }
    if(reply != null && reply.length > 0) {
        elm.append(
            $('<div>', {class: 'message message-reply'})
            .append($('<pre>', {class: 'siimple--my-0 siimple--pt-1'}).append(reply))
        );
    }
    elm.append($('<pre>').append(text));
    elm.click(function() {
        selectMessage(this);
    });
    area.append(elm);

    var scroll = $("#message_area_scroll");
    scroll.scrollTop(scroll[0].scrollHeight);
}

// in message area 
function pushStatus(text) {
    var area = $('#message_area');
    var elm = $('<div>', {class: 'status'});
    elm.append($('<span>', {class: 'siimple-tag siimple-tag--dark'}).append(text));
    area.append(elm);

    var scroll = $("#message_area_scroll");
    scroll.scrollTop(scroll[0].scrollHeight);
}

function unselectMessages() {
    $('.active').each(function() {
        $(this).removeClass('active');
    });
    $('#selected_clip').addClass('hidden');
}

function selectMessage(t) {
    var t = $(t);
    t.toggleClass('active');

    if($('.active').length == 0)
        $('#selected_clip').addClass('hidden');
    else
        $('#selected_clip').removeClass('hidden');
}

function selectedMessageToText() {
    var text = "";
    $('.active').each(function() {
        $(this).find('pre').each(function() {
            text += $(this).text() + '\n' 
        });
    });

    return text.trim();
}

function prepareReply() {
    var text = selectedMessageToText();
    var el = $('#reply_clip');
    el.removeClass('hidden');
    el.attr('msg', text);
    $('#reply_clip > span').text(text.substr(0, 15)+ '...');
    unselectMessages();
}

function send() {
    var text = $('#send_box').val().trim();
    if(text.length == 0) return;
    socket.send(JSON.stringify({
        cmd: "text",
        text: text,
        reply: $('#reply_clip').attr('msg')
    }));
    $('#send_box').val('');
    $('#reply_clip').attr('msg', '');
    $('#reply_clip').addClass('hidden');
    $('#reply_clip > span').text('');
}

function copyMessagesToClipboard() {
    var $temp = $("<textarea>");
    $("body").append($temp);
    $temp.val(selectedMessageToText()).select();
    document.execCommand("copy");
    $temp.remove();
    unselectMessages();
}

function cleanMessage() {
    $('#message_area').empty();
    $('#action_clip').addClass('hidden');
}

function vayaktiList() {
    var v = $('#vayakti_list');
    v.empty();
    Object.keys(vayaktiList).forEach((key) => {
        v.append($('<div>', {class: 'siimple-table-row'})
            .append($('<div>', {class: 'siimple-table-cell'}).append(key))
            .append($('<div>', {class: 'siimple-table-cell'}).append(vayaktiList[key])));
    });
    $('#vayakti_model').removeClass('hidden');
    $('#action_clip').addClass('hidden');
}
