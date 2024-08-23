import { Injectable } from '@angular/core';
import html2canvas from 'html2canvas';
import { jsPDF } from 'jspdf';

@Injectable({
  providedIn: 'root'
})
export class PdfService {

  constructor() { }

  public static async getAsPdfDoc(element: HTMLElement): Promise<jsPDF> {
    const documentStyle = getComputedStyle(document.documentElement);
    const bgColor = documentStyle.getPropertyValue('--surface-d');
    
    const canvas = await html2canvas(element, { scale: 3, backgroundColor: bgColor }).then(canvas => { return canvas; });
    const image = canvas.toDataURL("image/png");
    const margin = 8;
    
    let orientation : 'l' | 'p' = element.scrollWidth > element.scrollHeight ? 'l': 'p';
    const pdfSize = [element.scrollWidth+(margin*2), element.scrollHeight+(margin*2)];
    const imageSizing = [element.scrollWidth, element.scrollHeight]
    
    const doc = new jsPDF(orientation, 'px', pdfSize, true);
    doc.addImage(image, "png", margin, margin, imageSizing[0], imageSizing[1]);
    return doc;
  }

  /**
   * O nome padrão do pdf é 'document.pdf', ele fica dentro da pasta de downloads do sistema
   * @param element Elemento HTML que vai ser exportado como pdf 
   */

  public static async saveAsPdf(element: HTMLElement): Promise<void> {
    console.log(element);
    const doc = await this.getAsPdfDoc(element);
    const downlaodButton = document.createElement('a');
    downlaodButton.href=doc.output('datauristring');
    downlaodButton.download='document.pdf';
    downlaodButton.click();
    document.body.removeChild(downlaodButton);
  }

}
