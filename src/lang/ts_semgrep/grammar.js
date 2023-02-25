module.exports = grammar({
    name: 'semgrep',

    rules: {
        pattern: $ => repeat1(choice(
            $.ellipsis,
            $.metavar,
            // $.deep,
            $.text,
        )),
        ellipsis: $ => '...',
        metavar: $ => /\$_/,
        // metavar: $ => /\$[A-Z0-9_]+/,
        // deep: $ => seq('<...', $.pattern ,'...>')
    },

    externals: $ => [$.text],
});
