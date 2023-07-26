let nextPhotoTimeout;
let nextImage = {
  blob: null,
  name: null,
};

function imageError() {
  const viewpane = document.querySelector(".viewpane");
  viewpane.classList.add("fade-out");

  if (nextPhotoTimeout) {
    clearTimeout(nextPhotoTimeout);
  }

  nextPhotoTimeout = setTimeout(() => getNextPhoto(), 1000);
}

function preloadPhoto() {
  const root = window.location.host;
  const photoUrl = "http://" + root + "/next";

  fetch(photoUrl)
    .then((res) => {
      const name = res.headers.get("content-disposition");

      if (name !== nextImage.name) {
        nextImage.name = name;
        return res.blob();
      }

      return null;
    })
    .then((blob) =>
      blob && blob.type.startsWith("image/heic") ? heic2any({ blob }) : blob
    )
    .then((blob) => {
      if (blob) {
        nextImage.blob = URL.createObjectURL(blob);
      }
    });
}

preloadPhoto();
setInterval(() => preloadPhoto(), 2000);

function getNextPhoto() {
  const viewpane = document.querySelector("img.viewpane");

  if (nextPhotoTimeout) {
    clearTimeout(nextPhotoTimeout);
  }

  if (!nextImage.blob || nextImage.blob == viewpane.src) {
    nextPhotoTimeout = setTimeout(() => getNextPhoto(), 1000);
    return;
  }

  viewpane.classList.add("fade-out");

  setTimeout(() => {
    const filename = nextImage.name;
    viewpane.src = nextImage.blob;
    viewpane.onload = function () {
      viewpane.classList.remove("fade-out");
    };
    viewpane.onerror = function () {
      console.error(`File [${filename}] failed to load.`);
    };
    nextPhotoTimeout = setTimeout(() => getNextPhoto(), 3750);
  }, 250);
}

document.addEventListener("DOMContentLoaded", function (event) {
  getNextPhoto();

  setTimeout(() => {
    document.querySelector(".fullscreen-button").classList.add("hidden");
  }, 3000);
});
