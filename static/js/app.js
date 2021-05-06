
let State = class {
    static login() {
        $('#connect_panel').removeClass('is-hidden');
        $('#chat_panel').addClass('is-hidden');
        $('#reply_clip').addClass('is-hidden');
        $('#selected_clip').addClass('is-hidden');
        $('#action_clip').addClass('is-hidden');
        $('#vayakti_model').addClass('is-hidden');
        $('[name="error_msg"]').addClass('is-hidden');
    }

    static chat() {
        $('#chat_panel').removeClass('is-hidden');
        $('#connect_panel').addClass('is-hidden');
        $('#reply_clip').addClass('is-hidden');
        $('#selected_clip').addClass('is-hidden');
        $('#action_clip').addClass('is-hidden');
        $('#vayakti_model').addClass('is-hidden');
        $('#next_btn').addClass('is-hidden');
        $('#send_box').focus();
    }

    static showProgress() {
        $('#progress').removeClass('is-hidden');
    }
    
    static hideProgress() {
        $('#progress').addClass('is-hidden');
    }
};

let Messages = class {
    static pick(elm) {
        var elm = $(elm);
        elm.toggleClass('active');

        if($('.active').length == 0)
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
        typing.forEach((val)  => {
            text += val + ','
        })
        text = text.substr(0, text.length-1);
        text += ' is typing...'
        $('#status_area').append($('<div>', { id: 'typing' }).append(text));

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    static pushMessage(sender, text, reply = null) {
        var isMe = myinfo.kunjika == sender;
        var area = $('#message_area');
        var elm = $('<div>', {class: 'message '+(isMe?'message-me':'message-other')});
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
            selectMessage(this);
        });
        area.append(elm);

        var scroll = $("#message_area_scroll");
        scroll.scrollTop(scroll[0].scrollHeight);
    }

    // in message area 
    static pushStatus(text) {
        var area = $('#message_area');
        var elm = $('<div>', {class: 'status'});
        elm.append($('<small>', {class: 'tag bg-light'}).append(text));
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
        unselectMessages();
    }
    
    static copyMessagesToClipboard() {
        var $temp = $("<textarea>");
        $("body").append($temp);
        $temp.val(this.selectedMessageToText()).select();
        document.execCommand("copy");
        $temp.remove();
        unselectMessages();
    }
    
    static cleanMessage() {
        $('#message_area').empty();
        $('#status_area').empty();
        $('#action_clip').addClass('is-hidden');
    }

    static currentTime() {
        var today = new Date();
        return today.getHours()+':'+today.getMinutes();
    }
}

class Actions {
    actions = []; // [[id, func]]

    execute() {
        if(this.actions.length <= 0) return;

        var act = this.actions[0];
        this.actions.shift();

        act[1]();
    }

    clear() {
        this.actions = [];
    }

    clear_key(ac) {
        this.actions = this.actions.filter(function (arr) {
            return arr[0] != ac
        });
    }

    has_key(ac) {
        var out = this.actions.find(function (arr) {
            return arr[0] == ac
        });
        return out != undefined;
    }

    add(id, func) {
        this.actions.push([id, func]);
    }
}

var actions = new Actions();

$(document).ready(() => {
    $(".tabs > a").click(function() {
        var t = $(this);
        $(t.parents('tabs').first()).find('form').each(function(i,elm) {
            var elm = $(elm);
            if(elm.attr('name') == t.attr('name')) 
                elm.removeClass('is-hidden');
            else elm.addClass('is-hidden');
        });
        $(".tabs > a").each(function(i,elm) {
            var elm = $(elm);
            if(elm.attr('name') != t.attr('name')) 
                elm.removeClass('active');
            else elm.addClass('active');
        });
    });

    $('.message-me, .message-other').click(function() {
        Messages.select();
    });

    Messages.setupTyping();

    $('[name=connect]').click(function () {
        State.showProgress();
        connect($(this).parents('form').first());
    });

    var textarea = $('#send_box')[0];

    textarea.addEventListener('keydown', autosize);
                
    function autosize(){
        var el = this;
        setTimeout(function(){
            el.style.cssText = 'height:auto; padding:0';
            el.style.cssText = 'height:' + el.scrollHeight + 'px';
        },0);
    }
});

// Create WebSocket connection.
const socket = new WebSocket('ws://'+window.location.host+'/ws/');
var myinfo = {
    kunjika: "",
    name: ""
};
var vayakti = {};
var typing = [];
var no_name_message = false;

// Connection opened
socket.addEventListener('open', function (event) {
    var params = window.location.search;
    params = params.substr(1,params.length).split('&');
    
    if(params.length < 3) {
        State.hideProgress();
        return;
    }

    var frm = $('form[name=kaksh_sec]');
    frm.find('[name=kaksh_kunjika]').val(params[0]);
    frm.find('[name=kunjika]').val(params[1]);
    frm.find('[name=name]').val(params[2]);
    
    connect(frm);
});

// Listen for messages
socket.addEventListener('message', function (event) {
    var j = JSON.parse(event.data);
    switch(j.cmd) {
        case 'resp':
            if(j.result == 'Err') {
                if($('#chat_panel').hasClass('is-hidden')) {
                    $('[name="error_msg"]').text(j.message);
                    $('[name="error_msg"]').removeClass('is-hidden');
                    State.hideProgress();
                    actions.clear_key('join');
                } else {
                    Messages.pushStatus(j.message);
                }
            } else if(j.result == 'Ok'){
                actions.execute();
            }
            break;
        case 'kunjika':
            myinfo.kunjika = j.kunjika;
            break;
        case 'random':
            actions.execute();
            actions.clear_key('join');
            $('#next_btn').removeClass('is-hidden');
            no_name_message = true;
            Messages.pushStatus('Say hi to '+j.name);
            break;
        case 'status':
            if(j.status == "typing") {
                typing.push(j.kunjika);
                Messages.pushTypingStatus();
            } else if(j.status == "typing_end") {
                const index = typing.indexOf(j.kunjika);
                if (index > -1) typing.splice(index, 1);
                Messages.pushTypingStatus();
            }
            break;
        case 'text':
            Messages.pushMessage(j.kunjika, j.text, j.reply);
            break;
        case 'connected':
            vayakti[j.kunjika] = j.name;
            if(!$('#vayakti_model').hasClass('.is-hidden')) refreshVayaktiList();
            Messages.pushStatus('Vyakti '+j.name+' connected as '+j.kunjika.substr(0,8)+' at '+Messages.currentTime());
            break;
        case 'disconnected':
            delete vayakti[j.kunjika];
            if(!$('#vayakti_model').hasClass('.is-hidden')) refreshVayaktiList();
            Messages.pushStatus('Vyakti '+j.name+' disconnected as '+j.kunjika.substr(0,8)+' at '+Messages.currentTime());
            break;
        case 'left':
            myinfo.kunjika = '';
            myinfo.name = '';
            State.login();
            break;
        case 'list':
            JSON.parse(j.vayakti).forEach(function(usr) {
                vayakti[usr[0]] = usr[1];
            });
            break;
    }
});

function connect(frm) {
    if(actions.has_key('join') || actions.has_key('leave')) return;
    var frm = $(frm);
    var data = {};
    frm.serializeArray().forEach(el => {
        if(typeof el.value == 'string')
            data[el.name] = el.value.trim();
        else
            data[el.name] = el.value;
    });

    if(data['length'] !== undefined) {
        data['length'] = parseInt(data['length']);
    }

    actions.add('join', function() {
        Messages.cleanMessage();
        myinfo.name = data.name;
        no_name_message = false;
        joining = false;
        vayakti = [];
        typing = [];
        State.chat();
        State.hideProgress();
        Messages.pushStatus('Connected as '+data.name+' at '+Messages.currentTime());
        socket.send(JSON.stringify({cmd: 'list'}));
    })

    data = Object.assign({cmd: frm.attr('cmd')}, data);
    socket.send(JSON.stringify(data));
}

function connect_next() {
    if(actions.has_key('join') || actions.has_key('leave')) return;
    State.showProgress();
    actions.add('join', function() {
        Messages.cleanMessage();
        State.chat();
        vayakti = [];
        typing = [];
        State.hideProgress();
        Messages.pushStatus('Connectedas '+data.name+' at '+Messages.currentTime());
        socket.send(JSON.stringify({cmd: 'list'}));
    });
    socket.send(JSON.stringify({ cmd: 'randnext' }));
}

function leave() {    
    if(actions.has_key('leave')) return;
    actions.clear();
    actions.add('leave', function() {
        myinfo.kunjika = '';
        myinfo.name = '';
        State.login();
        State.hideProgress();
    });
    socket.send(JSON.stringify({cmd: 'leave'}));
}

function sendTyping() {
    socket.send(JSON.stringify({
        cmd: 'status',
        status: 'typing'
    }));
}

function sendTypingEnd() {
    socket.send(JSON.stringify({
        cmd: 'status',
        status: 'typing_end'
    }));
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
    $('#reply_clip').addClass('is-hidden');
    $('#reply_clip > span').text('');
}

function vayaktiList() {
    refreshVayaktiList();
    $('#vayakti_model').removeClass('is-hidden');
    $('#action_clip').addClass('is-hidden');
}

function refreshVayaktiList() {
    var v = $('#vayakti_list');
    v.empty();
    Object.keys(vayakti).forEach((key) => {
        v.append($('<tr>')
            .append($('<td>').append(vayakti[key]))
            .append($('<td>').append(key)));
    });
}
