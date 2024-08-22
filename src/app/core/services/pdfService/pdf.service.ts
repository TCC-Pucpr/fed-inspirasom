import { Injectable } from '@angular/core';
import html2canvas from 'html2canvas';
import { jsPDF } from 'jspdf';

@Injectable({
  providedIn: 'root'
})
export class PdfService {

  constructor() { }

  public static saveAsPdf(element: HTMLElement) {

    const documentStyle = getComputedStyle(document.documentElement);
    const bgColor = documentStyle.getPropertyValue('--surface-d');
    
    html2canvas(element, { scale: 3, backgroundColor: bgColor }).then(canvas => {
      const image = canvas.toDataURL("image/png");
      const margin = 8;
      
      let orientation : 'l' | 'p' = element.scrollWidth > element.scrollHeight ? 'l': 'p';
      const pdfSize = [element.scrollWidth+(margin*2), element.scrollHeight+(margin*2)];
      const imageSizing = [element.scrollWidth, element.scrollHeight]

      console.log(pdfSize);
      
      const doc = new jsPDF(orientation, 'px', pdfSize, true);
      doc.addImage(image, "png", margin, margin, imageSizing[0], imageSizing[1]);
      doc.save();
    });
  }

}
