/**
 * Comprehensive Rust Enhancements
 * 
 * This file contains custom functionality for the Comprehensive Rust book
 * that extends the default mdbook behavior. By keeping these in a separate
 * file, we minimize the patches needed for book.js.
 */

(function comprehensiveRustEnhancements() {
    "use strict";

    /**
     * Track playground usage with Google Analytics
     * @param {boolean} modified - Whether playground code was modified
     * @param {string|null} error - Error type if any
     * @param {number} latency - Execution time in seconds
     */
    function trackPlaygroundUsage(modified, error, latency) {
        if (typeof gtag !== 'undefined') {
            gtag("event", "playground", {
                "modified": modified,
                "error": (error == null) ? null : 'compilation_error',
                "latency": latency,
            });
        }
    }

    /**
     * Add unused lint suppression to Rust code if warnunused class is not present
     * @param {string} text - The code text
     * @param {DOMTokenList} classes - CSS classes from the code element
     * @returns {string} - Modified or original text
     */
    function addUnusedLintSuppression(text, classes) {
        // Unless the code block has `warnunused`, allow all "unused" lints to avoid cluttering
        // the output.
        if (!classes.contains("warnunused")) {
            return '#![allow(unused)] ' + text;
        }
        return text;
    }

    /**
     * Check if a playground has been modified
     * @param {HTMLElement} playground - The playground element
     * @returns {boolean} - True if modified
     */
    function isPlaygroundModified(playground) {
        let code_block = playground.querySelector("code");
        if (window.ace && code_block.classList.contains("editable")) {
            let editor = window.ace.edit(code_block);
            return editor.getValue() != editor.originalCode;
        } else {
            return false;
        }
    }

    /**
     * Enhanced error handling for playground requests
     * @param {Response} response - Fetch response
     * @param {HTMLElement} result_block - Result display element
     * @param {HTMLElement} result_stderr_block - Stderr display element
     */
    function handlePlaygroundResponse(response, result_block, result_stderr_block) {
        if (response.error != null && response.error != '') {
            // output the error if there's any. e.g. timeout
            result_block.innerText = response.error;
            result_block.classList.remove("result-no-output");
            return;
        }

        if (response.stdout.trim() === '') {
            result_block.innerText = "No output";
            result_block.classList.add("result-no-output");
        } else {
            result_block.innerText = response.stdout;
            result_block.classList.remove("result-no-output");
        }

        // trim compile message
        // ====================
        // Compiling playground v0.0.1 (/playground)
        // Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
        // Running `target/debug/playground`
        // ====================
        const compileMsgRegex = /^\s+Compiling(.+)\s+Finished(.+)\s+Running(.+)\n/;
        response.stderr = response.stderr.replace(compileMsgRegex, "");
        if (response.stderr.trim() !== '') {
            result_stderr_block.classList.remove("hidden");
            result_stderr_block.innerText = response.stderr;
        }
    }

    /**
     * Get enhanced playground parameters with support for newer Rust editions
     * @param {string} text - Code text
     * @param {DOMTokenList} classes - CSS classes
     * @returns {object} - Parameters for playground API
     */
    function getPlaygroundParams(text, classes) {
        let edition = "2015";
        if (classes.contains("edition2018")) {
            edition = "2018";
        } else if (classes.contains("edition2021")) {
            edition = "2021";
        } else if (classes.contains("edition2024")) {
            edition = "2024";
        }

        var params = {
            backtrace: true,
            channel: "stable",
            code: text,
            edition: edition,
            mode: "debug",
            tests: false,
            crateType: "bin",
        };

        // If the code block has no `main` but does have tests, run those.
        if (text.indexOf("fn main") === -1 && text.indexOf("#[test]") !== -1) {
            params.tests = true;
        }

        if (text.indexOf("#![feature") !== -1) {
            params.version = "nightly";
        }

        return params;
    }

    /**
     * Enhanced fetch with longer timeout for comprehensive rust
     * @param {string} url - URL to fetch
     * @param {object} options - Fetch options
     * @param {number} timeout - Timeout in milliseconds (default 15000)
     * @returns {Promise} - Fetch promise with timeout
     */
    function fetch_with_timeout(url, options, timeout = 15000) {
        return Promise.race([
            fetch(url, options),
            new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout))
        ]);
    }

    // Export functions for use by book.js patches
    window.comprehensiveRustEnhancements = {
        trackPlaygroundUsage,
        addUnusedLintSuppression,
        isPlaygroundModified,
        handlePlaygroundResponse,
        getPlaygroundParams,
        fetch_with_timeout
    };

    // Initialize any startup code if needed
    console.log('Comprehensive Rust enhancements loaded');
})();