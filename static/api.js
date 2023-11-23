document.getElementById("buttonTweet").addEventListener("click", postTweet);

function unescapeHTML(escapedHTML) {
	return escapedHTML.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&').replace(/&#x2f;/g, '/');
}
async function deleteTweet(id) {
	try {
		const response = await fetch("http://127.0.0.1:8888/tweet/" + id, {
			method: "DELETE"
		});
		if(!response.ok) {
			throw new Error("Network response was not ok.");
		}
	} catch(error) {
		console.error("There was a problem deleting the data:", error);
	}
	// reset animation
	var postTweetDiv = document.getElementById("tweetList");
	postTweetDiv.innerHTML = "";
	postTweetDiv.style.animation = "none";
	postTweetDiv.offsetHeight;
	postTweetDiv.style.animation = null;
	fetchTweets();
}
async function postTweet() {
	if(document.getElementById("textareaTweet").value != "") {
		try {
			const response = await fetch("http://127.0.0.1:8888/tweet", {
				method: "POST",
				body: JSON.stringify({
					message: document.getElementById("textareaTweet").value
				}),
				headers: {
					"Content-type": "application/json; charset=UTF-8",
				},
			});
			if(!response.ok) {
				throw new Error("Network response was not ok.");
			}
		} catch(error) {
			console.error("There was a problem sending the data:", error);
		}
		// reset animation
		var postTweetDiv = document.getElementById("tweetList");
		postTweetDiv.innerHTML = "";
		postTweetDiv.style.animation = "none";
		postTweetDiv.offsetHeight;
		postTweetDiv.style.animation = null;
		fetchTweets();
	}
}
async function fetchTweets() {
	try {
		const response = await fetch("http://127.0.0.1:8888/tweets");
		if(!response.ok) {
			throw new Error("Network response was not ok.");
		}
		document.getElementById("textareaTweet").value = "";
		const data = await response.json();
		const tweetList = document.getElementById("tweetList");
		data.forEach((tweet) => {
			const date = new Date(tweet.date);
			const formattedDate = date.toUTCString();
			const tweetItem = document.createElement("div");
			tweetItem.id = "id-" + tweet.id;
			tweetItem.classList.add("tweet");
			const tweetMessage = document.createElement("div");
			tweetMessage.classList.add("tweetMessage");
			tweetMessage.textContent = unescapeHTML(tweet.message);
			tweetItem.appendChild(tweetMessage);
			const tweetDate = document.createElement("div");
			tweetDate.classList.add("tweetDate");
			tweetDate.textContent = formattedDate;
			tweetItem.appendChild(tweetDate);
			const button = document.createElement("button");
			button.textContent = "delete";
			button.classList.add("buttonDelete");
			tweetItem.appendChild(button);
			tweetList.appendChild(tweetItem);
			button.addEventListener("click", function() {
				deleteTweet(tweet.id);
			});
		});
	} catch(error) {
		console.error("There was a problem fetching the data:", error);
	}
}
window.onload = fetchTweets;