import { AfterViewInit, Component } from '@angular/core';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';
import { ButtonModule } from 'primeng/button';
import { DynamicDialogConfig, DynamicDialogRef } from 'primeng/dynamicdialog';
import Cropper from 'cropperjs';

@Component({
  selector: 'app-cropper',
  standalone: true,
  imports: [
    ButtonModule
  ],
  templateUrl: './cropper.component.html',
  styleUrl: './cropper.component.scss'
})
export class CropperComponent implements AfterViewInit {

  protected imageUrl!: SafeUrl;
  protected cropper!: Cropper;

  constructor(
    private config: DynamicDialogConfig,
    private dialogRef: DynamicDialogRef,
    private sanitizer: DomSanitizer
  ) {
    this.imageUrl = this.sanitizer.bypassSecurityTrustUrl(config.data.url);
  }

  public ngAfterViewInit() {
    this.initCropper();
  }

  protected initCropper() {
    const image = document.getElementById('image') as HTMLImageElement;
    this.cropper = new Cropper(image, {
      aspectRatio: 1,
      viewMode: 2,
      guides: false,
      dragMode: 'move'
    });
  }

  // make the crop box rounded
  // desnecessario, pois a imagem ja vai ser cortada quando for mostrada, 
  // porém ajuda com o tamanho, vai reduzir a qntd de dados que vão ser guardados
  protected getCanvas(sourceCanvas: any) {
    var canvas = document.createElement('canvas');
    var context: any = canvas.getContext('2d');
    var width = sourceCanvas.width;
    var height = sourceCanvas.height;

    canvas.width = width;
    canvas.height = height;
    context.imageSmoothingEnabled = true;
    context.drawImage(sourceCanvas, 0, 0, width, height);
    context.globalCompositeOperation = 'destination-in';
    context.beginPath();
    context.arc(
      width / 2,
      height / 2,
      Math.min(width, height) / 2,
      0,
      2 * Math.PI,
      true
    );
    context.fill();
    return canvas;
  }

  //get the cropped image and closes the dialog 
  //returning an url or null if no image
  protected crop() {
    const croppedCanvas = this.cropper.getCroppedCanvas({width: 300, height: 300});
    const roundedCanvas = this.getCanvas(croppedCanvas);

    let roundedImage = document.createElement('img');

    if (roundedImage) {
      this.dialogRef.close(roundedCanvas.toDataURL());
    } else {
      return this.dialogRef.close(null);
    }
  }

  // resets the cropper
  protected reset(){
    this.cropper.clear();
    this.cropper.crop();
    this.cropper.zoomTo(0);
    const containerData = this.cropper.getContainerData();
    const imageData = this.cropper.getImageData();
    const calculatedX = (containerData.width - imageData.width)/2;
    const calculatedY = (containerData.height - imageData.height)/2;
    this.cropper.moveTo(calculatedX, calculatedY);
  }

}
