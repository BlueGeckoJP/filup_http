function onClickSubmitButton() {
	const file = document.getElementById("fake-input-file").files[0];
	const formData = new FormData();
	formData.append("file", file);
	const action = "/api/upload";
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

function onChangeFileChooser() {
	const valInput = document.getElementById("fake-input-file");
	const filename = document.getElementById("filename");
	filename.innerHTML = valInput.value;
}
