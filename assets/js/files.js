function onClickRemove() {
	if (confirm("Are you sure you want to delete the file?")) {
		const filename = document.getElementById("filename").innerHTML;
		fetch("/api/remove", {
			method: "POST",
			body: filename,
		}).then((e) => {
			if (e.status === 200) {
				alert("OK");
			} else {
				alert("Status code was not normal: " + e.status);
			}
		});
	} else {
		alert("Aborted");
	}

	document.location.reload();
}

function onClickDownload(filepath) {
	let element = document.createElement("a");
	element.href = filepath;
	element.download = "";
	element.click();
}

function onDOMLoaded() {
	let item_list = document.getElementById("item-list");
	if (item_list.children.length === 0) {
		let element = document.createElement("p");
		element.innerHTML = "ファイルはありません";
		element.id = "not-found-msg";
		item_list.appendChild(element);
	}
}

if (document.readyState === "loading") {
	document.addEventListener("DOMContentLoaded", onDOMLoaded);
} else {
	onDOMLoaded();
}
