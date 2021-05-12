
var actions = new Actions();

// Create WebSocket connection.
var wsProtocol = 'ws://';
if (window.location.protocol === 'https:') {
    wsProtocol = 'wss://';
}
const socket = new WebSocket(wsProtocol+window.location.host+'/ws/');
var myinfo = {
    kunjika: "",
    name: ""
};
var vayakti = {};
var typing = [];
var no_name_message = false;

// check if support ws
if(!('WebSocket' in window || 'MozWebSocket' in window)) {
    $('#initerror').text('Warning: Web Browser dosen\'t support websocket! Upgrade');
}

// Connection opened
socket.onopen = function(event) {
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
    $('#progress_button').css('display', '');
    connect(frm);
}

// Connection fail
socket.onerror = function(event) {
    $('#initerror').text('Warning: Failed to connect websocket! Refresh the '+
        'page and if still don\'t work upgrade Web Browser');
}

// Listen for messages
socket.onmessage = function(event) {
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
            Messages.pushText(j.kunjika, j.text, j.reply, j.msg_id);
            break;
        case 'img':
            Messages.pushImage(j.kunjika, j.src,  j.msg_id);
            break;
        case 'react':
            Messages.addReaction(j.kunjika, j.emoji,  j.msg_id);
            break;
        case 'del':
            Messages.deleteMessages(j.msg_id);
            break;
        case 'edit':
            Messages.editMessages(j.msg_id, j.text);
            break;
        case 'connected':
            vayakti[j.kunjika] = j.name;
            if(!$('#vayakti_model').hasClass('.is-hidden')) refreshVayaktiList();
            Messages.pushStatus('Vyakti '+j.name+' connected as '+j.kunjika+' at '+Messages.currentTime());
            break;
        case 'disconnected':
            delete vayakti[j.kunjika];
            const index = typing.indexOf(j.kunjika);
            if (index > -1) {
                typing.splice(index, 1);
            }
            Messages.pushTypingStatus();
            if(!$('#vayakti_model').hasClass('.is-hidden')) refreshVayaktiList();
            Messages.pushStatus('Vyakti '+j.name+' disconnected as '+j.kunjika+' at '+Messages.currentTime());
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
}

function connect(frm) {
    if(actions.has_key('join') || actions.has_key('leave')) return;
    var frm = $(frm);
    var data = {};
    frm.serializeArray().forEach(function(el) {
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

        //push url
        var frm = $('form[name=kaksh_sec]');
        var url = '/?' + frm.find('[name=kaksh_kunjika]').val() + '&' +
        frm.find('[name=kunjika]').val() + '&' +
        frm.find('[name=name]').val();
        history.pushState({}, 'Lupt Chat', url);

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
        Messages.pushStatus('Connected as '+myinfo.name+' at '+Messages.currentTime());
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

    if($('#replyicon').hasClass('is-hidden')) {
        socket.send(JSON.stringify({
            cmd: "text",
            text: text,
            reply: $('#reply_clip').attr('msg')
        }));
    } else {
        socket.send(JSON.stringify({
            cmd: "edit",
            text: text,
            msg_id: $('#reply_clip').attr('msg')
        }));
        $('#replyicon').addClass('is-hidden');
    }
    $('#send_box').val('');
    $('#reply_clip').attr('msg', '');
    $('#reply_clip').addClass('is-hidden');
    $('#reply_clip > span').text('');
    autosize($('#send_box')[0]);
}


function sendReaction(emoji, msg_id) {
    socket.send(JSON.stringify({
        cmd: "react",
        msg_id: msg_id,
        emoji: emoji
    }));
    $('#react_bar').remove();
}

function deleteMessages() {
    var prop = {
        title: 'Delete Messages',
        text: 'Do you really want to delete?',
        checkLabel: 'Delete both side',
        check: true
    }

    dialog(prop, function() {
        if($('#dialog_check').prop('checked')) {
            var msg_id = [];
            $('.message.active').each(function() {
                msg_id.push($(this).attr('msgid'));
            });
            socket.send(JSON.stringify({
                cmd: "del",
                msg_id: msg_id
            }));
        } else $('.message.active').remove();
    });

    $('#selected_clip').addClass('is-hidden');
}

function vayaktiList() {
    refreshVayaktiList();
    $('#vayakti_model').removeClass('is-hidden');
    $('#action_clip').addClass('is-hidden');
}

function changeColor() {
    $('body').toggleClass('dark')
    $('#action_clip').addClass('is-hidden');
}

function refreshVayaktiList() {
    var v = $('#vayakti_list');
    v.empty();
    Object.keys(vayakti).forEach(function(key) {
        v.append($('<tr>')
            .append($('<td>').append(vayakti[key]))
            .append($('<td>').append(key)));
    });
}
           
function autosize(el){
    setTimeout(function(){
        el.style.cssText = 'height:auto; padding:0';
        el.style.cssText = 'height:' + el.scrollHeight + 'px';
        $('#reply_clip').css('bottom',  (el.scrollHeight + 20) + 'px');
        $('#selected_clip').css('bottom',  (el.scrollHeight + 30) + 'px');
    },0);    
}

// Dialog
var dialogCallback;
function dialog(prop, call) {
    dialogCallback = call;
    $('#dialog_title').text(prop.title);
    $('#dialog_text').text(prop.text);
    $('#dialog_check_label').text(prop.checkLabel);
    
    if(prop.check) $('#dialog_checkbox').removeClass('is-hidden');
    else $('#dialog_checkbox').addClass('is-hidden');

    $('#dialog').removeClass('is-hidden');
}

$('#dialog_cancel').click(function() {
    $('#dialog').addClass('is-hidden');
});

$('#dialog_ok').click(function() {
    $('#dialog').addClass('is-hidden');
    dialogCallback();
});

// Gif
$("#gif_search").keyup(function(event){
    if (event.key === 'Enter') {
        event.preventDefault();
        $('#gif_area').empty();
        positiongif = '_';
        querygif = this.value;
        loadGif();
    }
});

var positiongif = '_';
var querygif = '';
function loadGif() {
    var area = $('#gif_area');
    $.get('/gif/'+positiongif+'/'+querygif, function(data, status){
        if(status == 'success') {
            area.find('[name=more]').remove();
            positiongif = data.next;
            data.results.forEach(function(result) {
                var gif = result.media[0].tinygif.url;
                area.append($('<button>', {class: 'button', onclick: 'sendGif("'+encodeURI(gif)+'"); $("#gif_clip").addClass("is-hidden");'})
                    .append($('<img>', {src: gif})));
            });
            if(querygif != '') area.append($('<button>', {name: 'more', onclick: 'loadGif()', class: 'button', style: 'display: block'}).text('show more'));
        }
    });
}

function sendGif(gif) {
    socket.send(JSON.stringify({
        cmd: "img",
        src: gif
    }));
}