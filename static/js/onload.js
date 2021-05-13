if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches || getCookie('theme') == 'dark')  {
    $('body').toggleClass('dark');
}

$(document).ready(function() {
    
    $(".tabs > a").click(function() {
        var t = $(this);
        if(!this.hasAttribute('name')) return;
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

    textarea.addEventListener('keydown', function() {
        autosize(this);
    });
    
    autosize($('#send_box')[0]);

    $('#send_button').mousedown(function(evt) {
        evt.preventDefault();
        send();
    });

    Images.setupImages();
});

function setCookie(cname, cvalue, exdays) {
    var d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    var expires = "expires="+d.toUTCString();
    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}
  
function getCookie(cname) {
    var name = cname + "=";
    var ca = document.cookie.split(';');
    for(var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return "";
}