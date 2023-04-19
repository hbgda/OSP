const emailField = document.querySelector("#email");
const passField = document.querySelector("#password");
const rememberBox = document.querySelector("#remember");
const loginBtn = document.querySelector("#login-btn");

loginBtn.addEventListener("click", () => {
    if (!validateLoginForm()) return;

    fetch(CONFIG.API.LOGIN, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            email: emailField.value,
            password: passField.value,
        })
    }).then(async res => {
        let json = await res.json();
        if(!res.ok) {
            console.error(res.statusText, json)
            return
        }
        console.log(json, res.statusText)
        if (json["success"]) {
            window.location.href = "/"
        }
        else {
            alert(json["error"])
            input_err(emailField, "")
            input_err(passField, "")
        }
    })
});

function validateLoginForm() {

    if (!Validate.Input.Email(emailField.value)) {
        console.log("Invalid email.")
        input_err(emailField, "Invalid email.")
        return false
    }
    if (!Validate.Input.Password(passField.value)) {
        console.log("Invalid password.")
        input_err(passField, "Invalid password.")
        return false
    }

    return true
}