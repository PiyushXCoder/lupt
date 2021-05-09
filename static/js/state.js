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
