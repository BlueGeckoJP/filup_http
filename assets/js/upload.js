function onClickSubmitButton() {
	const form = document.getElementById("upload-form");
	const formData = new FormData(form);
	const action = form.getAttribute("action");
	const options = {
		method: "POST",
		body: formData,
	};
	fetch(action, options).then((e) => {
		if (e.status === 200) {
			alert("OK");
			return;
		}
		alert("Error");
	});
}
