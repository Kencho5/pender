var step = 1;

function changeStep() {
  document.querySelector(`#step${step}`).style.display = "none";
  document.querySelector(`#step${step + 1}`).style.display = "flex";

  step++;
  window.scrollTo({ top: 0, behavior: "smooth" });
}

async function compressImage(file, { quality = 1, type = file.type }) {
  const imageBitmap = await createImageBitmap(file);

  const canvas = document.createElement("canvas");
  canvas.width = imageBitmap.width;
  canvas.height = imageBitmap.height;
  const ctx = canvas.getContext("2d");
  ctx.drawImage(imageBitmap, 0, 0);

  const base64 = canvas.toDataURL(type, quality);

  return base64;
}

function selectChip(event) {
  const chip = event.target;
  const priceRow = document.querySelector(".price-row") || null;

  if (!chip.classList.contains("input-div")) {
    if (priceRow && chip.parentNode.getAttribute("input-name") == "post_type") {
      priceRow.style.display = "none";
      priceRow.querySelector("input").classList.remove("upload-input");
    }

    Array.from(chip.parentNode.children).forEach((element) => {
      element.classList.remove("active-upload-chip");
    });
    chip.classList.toggle("active-upload-chip");
  }

  if (priceRow && chip.id == "selling") {
    priceRow.style.display = "block";
    priceRow.querySelector("input").classList = "upload-input";
  }
}

function toggleAgeDropdown() {
  document.querySelector(".age-dropdown").classList.toggle("show-age");
}

function selectAgeType(event) {
  const age = event.target;

  if (age.classList.length == 0) {
    const ageSelector = document.querySelector(".age-selector").childNodes[0];
    const ageDropdown = document.querySelector(".age-dropdown");

    ageSelector.textContent = ` ${age.textContent} `;
    ageSelector.parentNode.id = age.id;
    ageDropdown.classList.remove("show-age");
  }
}

function validateForm() {
  let isValid = true;
  const msg = document.querySelector(".msg");
  const chips = document.querySelectorAll(".active-upload-chip");
  const inputs = document.querySelectorAll(".upload-input");
  const ageType = document.querySelector('[input-name="age-type"]');

  if (step == 1 && chips.length < 3) isValid = false;

  if (step == 2) {
    inputs.forEach((input) => {
      if (!input.value) isValid = false;
    });
    if (ageType && ageType.id == "all") isValid = false;
  }

  if (!isValid) {
    msg.style.display = "block";
  } else {
    msg.style.display = "none";
    changeStep();
  }
}

var body = {};
const fileInput = document.getElementById("fileInput");

fileInput.addEventListener("change", async (event) => {
  const imagesDiv = document.getElementById("imagesDiv");
  const files = event.target.files;

  for (const file of files) {
    const imageContainer = document.createElement("div");
    imageContainer.classList.add("image");

    // Create the first img element
    const img1 = document.createElement("img");
    img1.src = URL.createObjectURL(file);

    // Create the second img element (delete icon)
    const img2 = document.createElement("img");
    img2.src = "/assets/icons/cancel.svg";
    img2.classList.add("delete-icon");
    img2.alt = "Delete";

    imageContainer.appendChild(img1);
    imageContainer.appendChild(img2);

    imagesDiv.appendChild(imageContainer);

    const compressed = await compressImage(file, {
      quality: 0.6,
      type: "image/jpeg",
    });
    if (!body["photos"]) body["photos"] = [compressed];
    else body["photos"].push(compressed);

    img2.addEventListener("click", () => {
      imagesDiv.removeChild(imageContainer);

      const index = body["photos"].indexOf(compressed);
      if (index !== -1) {
        body["photos"].splice(index, 1);
      }
    });
  }
});

function upload() {
  if (!validateForm()) return;
}
