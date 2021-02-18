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
        activateMessage(this);
    });

    $('#selected_clip > .siimple-close').click(function() {
        deactivateMessages();
    });
});

// Create WebSocket connection.
const socket = new WebSocket('ws://localhost:8080/ws/');
var callbacks = [];
var myinfo = {
    kunjika: "",
    name: ""
};

// Connection opened
socket.addEventListener('open', function (event) {
    $('#progressbar').toggleClass('hidden');
});

// Listen for messages
socket.addEventListener('message', function (event) {
    var j = JSON.parse(event.data);
    console.log(j);
    switch(j.cmd) {
        case 'resp':
            if(j.result == 'Err') {
                console.log('Error', j.message);
            } else {
                if(callbacks.length > 0) {
                    callbacks[0]();
                    callbacks.splice(0);
                }
            }
            break;
        case 'text':
            pushMessage(j.kunjika, j.text, j.reply);
            break;
        
    }
});

function connect(frm) {
    var frm = $(frm);
    $('#progressbar').addClass('hidden');
    var data = {};
    frm.serializeArray().forEach(el => {
        data[el.name] = el.value;
    });

    callbacks.push(() => {
        socket.send(JSON.stringify(Object.assign({cmd: frm.attr('cmd')}, data)));
        $('#progressbar').addClass('hidden');
        $('#chat_panel').removeClass('hidden');
        myinfo.kunjika = data.kunjika;
        myinfo.name = data.name;
    });
    socket.send(JSON.stringify(Object.assign({cmd: 'seinfo'}, data)));
}

function leave() {
    callbacks.push(() => {
        $('#chat_panel').addClass('hidden');
        $('#reply_clip').addClass('hidden');
        $('#selected_clip').addClass('hidden');
        $('#action_clip').addClass('hidden');
    });
    socket.send(JSON.stringify({
        cmd: "leave"
    }));
}

function pushMessage(sender, text, reply = null) {
    var isMe = myinfo.kunjika == sender;
    var area = $('#message_area');
    var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other')});
    elm.append($('<div>', {class: 'message-by'}).append(sender));
    if(reply != null && reply.length > 0) {
        elm.append(
            $('<div>', {class: 'message message-reply'})
            .append($('<pre>', {class: 'siimple--my-0 siimple--pt-1'}).append(reply))
        );
    }
    elm.append($('<pre>').append(text));
    elm.click(function() {
        activateMessage(this);
    });
    area.append(elm)
}

function deactivateMessages() {
    $('.active').each(function() {
        $(this).removeClass('active');
    });
    $('#selected_clip').addClass('hidden');
}

function activateMessage(t) {
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
    deactivateMessages();
}

function send() {
    socket.send(JSON.stringify({
        cmd: "text",
        text: $('#send_box').val(),
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
    deactivateMessages();
}
// function wsend(p) {
//     socket.send(p);
// }

// function join(r, l) {
//     socket.send(JSON.stringify({
//         cmd: "join",
//         grih_kunjika: r,
//         length: l
//     }));
// }

// function leave() {
//     socket.send(JSON.stringify({
//         cmd: "leave"
//     }));
// }

// function send(t) {
//     socket.send(JSON.stringify({
//         cmd: "text",
//         text: t
//     }));
// }

// function info(k, n, t) {
//     socket.send(JSON.stringify({
//         cmd: "seinfo",
//         kunjika: k,
//         name: n,
//         tags: t
//     }));
// }

// function joinrand() {
//     socket.send(JSON.stringify({
//         cmd: "rand"
//     }));
// }