const EMAIL_LOCAL_PATTERN = /(?:^[^\s.]?(?:[a-zA-Z0-9!#$%&'*+\-\/=?^_`{|}~][\.]{0,1})*[^\s.]+)/
const EMAIL_DOMAIN_PATTERN = /[^\.\-@][a-zA-Z0-9\-]+[\.][a-zA-Z0-9\-]*[^\.\-]/

const PASSWORD_MIN_LENGTH = 7
const PASSWORD_MAX_LENGTH = 21
const PASSWORD_CHARS_PATTERN = /[a-zA-Z0-9]*/

const NAME_PATTERN = /[a-zA-Z'\-]*/

const Validate = {
    Input: {
        Name: (input) => {
            return (
                input.length > 0           &&
                NAME_PATTERN.test(input)
            )
        },
        Email: (input) => {
            let parts = input.split("@")
            if (parts.length != 2) {
                return false
            }
            let [local, domain] = parts
            return (
                EMAIL_LOCAL_PATTERN.test(local)     &&
                EMAIL_DOMAIN_PATTERN.test(domain)   
            )
        },
        Password: (input) => {
            return (
                input.length >= PASSWORD_MIN_LENGTH             &&  // Password is within length limits
                input.length <= PASSWORD_MAX_LENGTH             &&  // --

                input.toLowerCase() != input                    &&  // Password contains capital letter
                input.toUpperCase() != input                    &&  // Password contains lowercase letter

                input.replace(PASSWORD_CHARS_PATTERN, "") == ""     // Password only contains valid characters
            )
        }
    }
}

function input_err(input, err) {
    input.classList.add("error")
    setTimeout(() => {
        input.classList.remove("error");
    }, 3000)
}

function checkPasswordStrength(input) {
    // TODO
    // Basic length based strength for now
    return (
        input.length <= PASSWORD_MIN_LENGTH + 2 ? "low"  :
        input.length <= PASSWORD_MIN_LENGTH + 5 ? "mid"  :
        "high"
        // input.length <= PASSWORD_MIN_LENGTH + 7 ? "high" :
    )
}