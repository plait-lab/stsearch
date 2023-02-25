#include <ctype.h>
#include <tree_sitter/parser.h>

#define TS(NAME) tree_sitter_semgrep_##NAME
#define SCANNER(NAME) TS(external_scanner_##NAME)

#define UNUSED(X) __attribute__((unused)) unused_##X

enum TokenType {
  TEXT,
};

// Stateless, so mark unused
#define payload UNUSED(payload)

static bool is_holename(int32_t c);

bool SCANNER(scan)(void *payload, TSLexer *lexer, const bool *valid_symbols) {
  if (valid_symbols[TEXT]) {
    bool found = false;

    while (!lexer->eof(lexer)) {
      switch (lexer->lookahead) {
        case '<':  // might be deep
          lexer->advance(lexer, false);
          if (lexer->lookahead != '.') break;
        case '.':  // might be ellipsis
          lexer->advance(lexer, false);
          if (lexer->lookahead != '.') break;
          lexer->advance(lexer, false);
          if (lexer->lookahead != '.') break;
          goto done;
        case '$':  // might be metavar
          lexer->advance(lexer, false);
          if (!is_holename(lexer->lookahead)) break;
          goto done;
        default:
          lexer->advance(lexer, false);
      }
      lexer->mark_end(lexer);
      found = true;
    }

  done:
    if (found) {
      lexer->result_symbol = TEXT;
      return true;
    }
  }
  return false;
}

static bool is_holename(int32_t c) {
  return ('A' <= c && c <= 'Z') || ('0' <= c && c <= '9') || c == '_';
}

// Provide needed stubs
#define buffer UNUSED(buffer)
#define length UNUSED(length)

void *SCANNER(create)() { return NULL; }
unsigned SCANNER(serialize)(void *payload, char *buffer) { return 0; }
void SCANNER(deserialize)(void *payload, const char *buffer, unsigned length) {}
void SCANNER(destroy)(void *payload) {}
