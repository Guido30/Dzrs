export function parseFileName(fileData, template) {
  const fileName = template
    .replace("%title%", fileData.title)
    .replace("%album%", fileData.album_title)
    .replace("%artist%", fileData.performer_name);
  return fileName;
}

export default {};
