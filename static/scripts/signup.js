const firstnameField = document.querySelector("#firstname")
const surnameField = document.querySelector("#surname")
const emailField = document.querySelector("#email")
const passwordField = document.querySelector("#password")
const confirmPasswordField = document.querySelector("#confirm-password")
const passwordStrengthBar = document.querySelector("#password-strength")
const signupBtn = document.querySelector("#signup-btn")

signupBtn.addEventListener("click", () => {
    if (!validateSignupForm()) return;

    fetch(CONFIG.API.SIGNUP, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            firstname: firstnameField.value,
            surname: surnameField.value,
            email: emailField.value,
            password: passwordField.value
        })
    })
    .then(async res => {
        let json = await res.json()
        if (!res.ok) {
            console.error(res.statusText, json)
            return
        }
        console.log(json, res.statusText)
        if (json["success"]) {
            window.location.href = "/login"
        }
        else {
            alert(json["error"])
        }
    })
})

passwordField.addEventListener("input", () => {
    let strength = checkPasswordStrength(passwordField.value)
    console.log(strength)
    passwordStrengthBar.setAttribute("data-strength", strength)
})

function validateSignupForm() {
    
    if (!Validate.Input.Name(firstnameField.value)) {
        console.log("Invalid firstname.")
        input_err(firstnameField, "Invalid firstname.")
        return false
    }

    if (!Validate.Input.Name(surnameField.value)) {
        console.log("Invalid surname.")
        input_err(surnameField, "Invalid surname.")
        return false
    }

    if (!Validate.Input.Email(emailField.value)) {
        console.log("Invalid email.")
        input_err(emailField, "Invalid email.")
        return false
    }

    if (!Validate.Input.Password(passwordField.value)) {
        console.log("Invalid password.")
        input_err(passwordField, "Invalid password.")
        return false
    }

    if (passwordField.value != confirmPasswordField.value) {
        console.log("Passwords don't match.")
        input_err(confirmPasswordField, "Password doesn't match.")
        return false
    }

    return true
}