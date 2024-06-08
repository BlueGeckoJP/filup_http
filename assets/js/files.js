function onClickRemove(filename) {
	if (confirm("Are you sure you want to delete the file?")) {
		fetch("/api/remove", {
			method: "POST",
			body: filename,
		}).then((e) => {
			if (e.status === 200) {
				alert("OK");
			} else {
				alert("Status code was not normal | " + e.status);
			}
		});
	} else {
		alert("Aborted");
	}

	document.location.reload();
}
