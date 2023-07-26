document.addEventListener("DOMContentLoaded", function (event) {
  const viewpane = document.querySelector(".viewpane");
  const root = window.location.host;
  const photoUrl = 'http://' + root + "/next";

  function getNextPhoto() {
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
            setTimeout(() => getNextPhoto(), 4000);
          }, 250);
        }, 300);
      });
  }

  getNextPhoto();

  setTimeout(() => {
    document.querySelector(".fullscreen-button").classList.add("hidden");
  }, 3000);
});
