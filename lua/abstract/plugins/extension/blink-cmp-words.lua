--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: blink-cmp-words
Source: https://github.com/archie-judd/blink-cmp-words

A fast and offline word and synonym completion provider for Neovim
blink-cmp-words is an extension for blink-cmp that can be used in two ways:
    As a dictionary - provides word completion with definitions and related terms
    As a thesaurus - provides synonym completion for finding alternative words

It uses Princeton University's WordNet lexical database
to provide words, definitions and lexical relations.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local config = {

	-- Use the dictionary source
	dictionary = {
		name = "blink-cmp-words",
		module = "blink-cmp-words.dictionary",
		max_items = 5,
		-- All available options
		opts = {
			-- The number of characters required to trigger completion.
			-- Set this higher if completion is slow, 3 is default.
			dictionary_search_threshold = 4,

			score_offset = 100,
			pointer_symbols = { "!", "&", "^" },
		},
	},

	-- Use the thesaurus source
	thesaurus = {
		name = "blink-cmp-words",
		module = "blink-cmp-words.thesaurus",
		max_items = 6,
		-- All available options
		opts = {
			-- A score offset applied to returned items.
			-- By default the highest score is 0 (item 1 has a score of -1, item 2 of -2 etc..).
			score_offset = 9,

			-- Default pointers define the lexical relations listed under each definition,
			-- see Pointer Symbols below.
			-- Default is as below ("antonyms", "similar to" and "also see").
			pointer_symbols = { "!", "&", "^" },
		},
	},
}

return config
