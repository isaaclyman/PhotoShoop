let nextPhotoTimeout;

function imageError() {
  getNextPhoto();
}

function getNextPhoto() {
  const viewpane = document.querySelector(".viewpane");
  const root = window.location.host;
  const photoUrl = 'http://' + root + "/next";

  if (nextPhotoTimeout) {
    clearTimeout(nextPhotoTimeout);
  }

  fetch(photoUrl)
    .then((res) => res.blob())
    .then((blob) =>
      blob.type.startsWith("image/heic") ? heic2any({ blob }) : blob
    )
    .then((blob) => {
      const image = URL.createObjectURL(blob);
      viewpane.classList.add("fade-out");

      setTimeout(() => {
        viewpane.src = image;
        setTimeout(() => {
          viewpane.classList.remove("fade-out");
          nextPhotoTimeout = setTimeout(() => getNextPhoto(), 4000);
        }, 250);
      }, 300);
    });
}

document.addEventListener("DOMContentLoaded", function (event) {
  getNextPhoto();

  setTimeout(() => {
    document.querySelector(".fullscreen-button").classList.add("hidden");
  }, 3000);
});
