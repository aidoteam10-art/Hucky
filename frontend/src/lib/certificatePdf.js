const PAGE_WIDTH = 842;
const PAGE_HEIGHT = 595;
const CANVAS_WIDTH = 1684;
const CANVAS_HEIGHT = 1190;
const FIRST_PAGE_ROUNDS = 4;
const ROUNDS_PER_EXTRA_PAGE = 16;

export async function downloadCertificatePdf(certificate) {
	const canvases = [createCertificateCanvas(certificate)];
	const extraRoundPages = certificateRoundPages(certificate.rounds || []);

	if (extraRoundPages.length > 0) {
		extraRoundPages.forEach((rounds, pageIndex) => {
			canvases.push(createRoundsCanvas(certificate, rounds, pageIndex + 1));
		});
	}

	const logo = await loadImage('/icons/logo_black.png');
	for (const canvas of canvases) {
		drawLogo(canvas.getContext('2d'), logo);
	}

	const images = canvases.map((canvas) => ({
		bytes: dataUrlToBytes(canvas.toDataURL('image/jpeg', 0.95)),
		width: CANVAS_WIDTH,
		height: CANVAS_HEIGHT
	}));
	const pdfBytes = buildImagePdf(images);
	const blob = new Blob([pdfBytes], { type: 'application/pdf' });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = `certificate-${slugify(certificate.tournamentName)}.pdf`;
	document.body.appendChild(link);
	link.click();
	link.remove();
	URL.revokeObjectURL(url);
}

export function formatPlace(place) {
	if (place === null || place === undefined) return 'Не подано';
	return `${place} місце`;
}

export function certificateRoundPages(rounds) {
	if (!Array.isArray(rounds) || rounds.length <= FIRST_PAGE_ROUNDS) return [];
	const pages = [];
	for (let index = 0; index < rounds.length; index += ROUNDS_PER_EXTRA_PAGE) {
		pages.push(rounds.slice(index, index + ROUNDS_PER_EXTRA_PAGE));
	}
	return pages;
}

function createCertificateCanvas(certificate) {
	const canvas = document.createElement('canvas');
	canvas.width = CANVAS_WIDTH;
	canvas.height = CANVAS_HEIGHT;
	drawCertificate(canvas.getContext('2d'), certificate);
	return canvas;
}

function createRoundsCanvas(certificate, rounds, pageNumber) {
	const canvas = document.createElement('canvas');
	canvas.width = CANVAS_WIDTH;
	canvas.height = CANVAS_HEIGHT;
	drawRoundsPage(canvas.getContext('2d'), certificate, rounds, pageNumber);
	return canvas;
}

function drawBase(context) {
	context.fillStyle = '#F9FFE5';
	context.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

	context.strokeStyle = '#CCFF00';
	context.lineWidth = 28;
	context.strokeRect(58, 58, CANVAS_WIDTH - 116, CANVAS_HEIGHT - 116);

	context.strokeStyle = '#89AB00';
	context.lineWidth = 4;
	context.strokeRect(92, 92, CANVAS_WIDTH - 184, CANVAS_HEIGHT - 184);
}

function drawLogo(context, logo) {
	if (logo) {
		context.drawImage(logo, 108, 78, 86, 86);
	}

	context.fillStyle = '#191F00';
	context.textAlign = 'left';
	context.font = '700 46px Inter, Arial, sans-serif';
	context.fillText('Hucky', 208, 133);
}

function drawCertificate(context, certificate) {
	drawBase(context);

	context.fillStyle = '#191F00';
	context.textAlign = 'center';
	context.font = '800 86px Inter, Arial, sans-serif';
	context.fillText('СЕРТИФІКАТ', CANVAS_WIDTH / 2, 245);

	context.font = '500 34px Inter, Arial, sans-serif';
	context.fillStyle = '#696969';
	context.fillText('підтверджує, що', CANVAS_WIDTH / 2, 310);

	context.fillStyle = '#191F00';
	context.font = '800 72px Inter, Arial, sans-serif';
	wrapCenteredText(context, certificate.userName, CANVAS_WIDTH / 2, 400, 1180, 80);

	context.font = '500 34px Inter, Arial, sans-serif';
	context.fillStyle = '#696969';
	wrapCenteredText(
		context,
		`брав(ла) участь у команді «${certificate.teamName}» у турнірі`,
		CANVAS_WIDTH / 2,
		500,
		1260,
		46
	);

	context.fillStyle = '#191F00';
	context.font = '700 42px Inter, Arial, sans-serif';
	wrapCenteredText(context, `«${certificate.tournamentName}»`, CANVAS_WIDTH / 2, 585, 1220, 52);

	context.fillStyle = '#CCFF00';
	roundRect(context, 502, 660, 680, 90, 24);
	context.fill();

	context.fillStyle = '#191F00';
	context.font = '800 42px Inter, Arial, sans-serif';
	context.fillText(`Загальне місце: ${formatPlace(certificate.overallPlace)}`, CANVAS_WIDTH / 2, 718);

	drawRoundList(context, (certificate.rounds || []).slice(0, FIRST_PAGE_ROUNDS), 835);

	if ((certificate.rounds || []).length > FIRST_PAGE_ROUNDS) {
		context.textAlign = 'center';
		context.fillStyle = '#696969';
		context.font = '500 24px Inter, Arial, sans-serif';
		context.fillText('Повний список результатів по раундах додано на наступних сторінках', CANVAS_WIDTH / 2, 1040);
	}

	drawFooter(context, certificate.issuedAt);
}

function drawRoundsPage(context, certificate, rounds, pageNumber) {
	drawBase(context);

	context.fillStyle = '#191F00';
	context.textAlign = 'center';
	context.font = '800 58px Inter, Arial, sans-serif';
	context.fillText('РЕЗУЛЬТАТИ ПО РАУНДАХ', CANVAS_WIDTH / 2, 235);

	context.fillStyle = '#696969';
	context.font = '500 30px Inter, Arial, sans-serif';
	wrapCenteredText(context, `«${certificate.tournamentName}»`, CANVAS_WIDTH / 2, 300, 1180, 40);

	drawRoundList(context, rounds, 395);

	context.textAlign = 'center';
	context.fillStyle = '#696969';
	context.font = '500 22px Inter, Arial, sans-serif';
	context.fillText(`Сторінка результатів ${pageNumber}`, CANVAS_WIDTH / 2, 1080);
}

function drawRoundList(context, rounds, startY) {
	context.fillStyle = '#191F00';
	context.textAlign = 'left';
	context.font = '700 34px Inter, Arial, sans-serif';
	context.fillText('Результати по раундах', 300, startY);

	context.font = '500 28px Inter, Arial, sans-serif';
	const listStartY = startY + 55;
	const lineHeight = 46;
	rounds.forEach((round, index) => {
		const y = listStartY + index * lineHeight;
		context.fillStyle = index % 2 === 0 ? '#FFFFFF' : '#F4F4F4';
		roundRect(context, 300, y - 30, 1084, 38, 10);
		context.fill();
		context.fillStyle = '#191F00';
		fitText(context, round.title, 328, y, 780);
		context.textAlign = 'right';
		context.fillText(formatPlace(round.place), 1350, y);
		context.textAlign = 'left';
	});
}

function drawFooter(context, issuedAt) {
	context.textAlign = 'left';
	context.fillStyle = '#696969';
	context.font = '500 24px Inter, Arial, sans-serif';
	context.fillText(`Дата видачі: ${issuedAt}`, 300, 1080);

	context.textAlign = 'right';
	context.fillStyle = '#191F00';
	context.font = '700 30px Inter, Arial, sans-serif';
	context.fillText('A.I.D.O. Team', 1384, 1080);
}

function wrapCenteredText(context, text, x, y, maxWidth, lineHeight) {
	const words = String(text || '').split(' ');
	let line = '';
	let currentY = y;

	for (const word of words) {
		const testLine = line ? `${line} ${word}` : word;
		if (context.measureText(testLine).width > maxWidth && line) {
			context.fillText(line, x, currentY);
			line = word;
			currentY += lineHeight;
		} else {
			line = testLine;
		}
	}

	if (line) context.fillText(line, x, currentY);
}

function fitText(context, text, x, y, maxWidth) {
	const value = String(text || '');
	if (context.measureText(value).width <= maxWidth) {
		context.fillText(value, x, y);
		return;
	}

	let shortened = value;
	while (shortened.length > 1 && context.measureText(`${shortened}...`).width > maxWidth) {
		shortened = shortened.slice(0, -1);
	}
	context.fillText(`${shortened}...`, x, y);
}

function roundRect(context, x, y, width, height, radius) {
	context.beginPath();
	context.moveTo(x + radius, y);
	context.arcTo(x + width, y, x + width, y + height, radius);
	context.arcTo(x + width, y + height, x, y + height, radius);
	context.arcTo(x, y + height, x, y, radius);
	context.arcTo(x, y, x + width, y, radius);
	context.closePath();
}

function loadImage(src) {
	return new Promise((resolve) => {
		const image = new Image();
		image.onload = () => resolve(image);
		image.onerror = () => resolve(null);
		image.src = src;
	});
}

function dataUrlToBytes(dataUrl) {
	const base64 = dataUrl.split(',')[1];
	const binary = atob(base64);
	const bytes = new Uint8Array(binary.length);
	for (let index = 0; index < binary.length; index += 1) {
		bytes[index] = binary.charCodeAt(index);
	}
	return bytes;
}

function buildImagePdf(images) {
	const encoder = new TextEncoder();
	const chunks = [];
	const offsets = [];
	let length = 0;
	const objectCount = 2 + images.length * 3;

	const addText = (text) => addBytes(encoder.encode(text));
	const addBytes = (bytes) => {
		chunks.push(bytes);
		length += bytes.length;
	};
	const addObject = (id, body) => {
		offsets[id] = length;
		addText(`${id} 0 obj\n${body}\nendobj\n`);
	};

	addText('%PDF-1.4\n');
	addObject(1, '<< /Type /Catalog /Pages 2 0 R >>');
	const pageIds = images.map((_, index) => 3 + index * 3);
	addObject(2, `<< /Type /Pages /Kids [${pageIds.map((id) => `${id} 0 R`).join(' ')}] /Count ${images.length} >>`);

	images.forEach((image, index) => {
		const pageId = 3 + index * 3;
		const contentId = pageId + 1;
		const imageId = pageId + 2;
		addObject(
			pageId,
			`<< /Type /Page /Parent 2 0 R /MediaBox [0 0 ${PAGE_WIDTH} ${PAGE_HEIGHT}] /Resources << /XObject << /Im${index} ${imageId} 0 R >> >> /Contents ${contentId} 0 R >>`
		);
		const pageContent = `q\n${PAGE_WIDTH} 0 0 ${PAGE_HEIGHT} 0 0 cm\n/Im${index} Do\nQ`;
		addObject(contentId, `<< /Length ${pageContent.length} >>\nstream\n${pageContent}\nendstream`);

		offsets[imageId] = length;
		addText(
			`${imageId} 0 obj\n<< /Type /XObject /Subtype /Image /Width ${image.width} /Height ${image.height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length ${image.bytes.length} >>\nstream\n`
		);
		addBytes(image.bytes);
		addText('\nendstream\nendobj\n');
	});

	const xrefOffset = length;
	addText(`xref\n0 ${objectCount + 1}\n0000000000 65535 f \n`);
	for (let id = 1; id <= objectCount; id += 1) {
		addText(`${String(offsets[id]).padStart(10, '0')} 00000 n \n`);
	}
	addText(`trailer\n<< /Size ${objectCount + 1} /Root 1 0 R >>\nstartxref\n${xrefOffset}\n%%EOF`);

	const pdfBytes = new Uint8Array(length);
	let offset = 0;
	for (const chunk of chunks) {
		pdfBytes.set(chunk, offset);
		offset += chunk.length;
	}
	return pdfBytes;
}

function slugify(value) {
	return String(value || 'certificate')
		.toLowerCase()
		.replace(/[^a-z0-9а-яіїєґ]+/gi, '-')
		.replace(/^-+|-+$/g, '');
}
