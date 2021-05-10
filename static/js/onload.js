if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
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

    Camera.setupCamera();
});