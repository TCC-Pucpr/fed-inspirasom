import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { DialogService, DynamicDialogRef } from 'primeng/dynamicdialog';
import { FileUploadModule } from 'primeng/fileupload';
import { CropperComponent } from './components/cropper/cropper.component';
import { DataService, StorageKeys } from '../../services/dataService/data.service';

@Component({
  selector: 'app-user-profile',
  standalone: true,
  imports: [
    CommonModule,
    HttpClientModule,
    FileUploadModule
  ],
  templateUrl: './user-profile.component.html',
  styleUrl: './user-profile.component.scss'
})
export class UserProfileComponent implements OnInit {

  public readonly defaultImage: string = "/assets/default-user.jpg";
  public currentImage: string = "";
  public readonly PFP_KEY = "PFP_KEY";

  private dialogRef: DynamicDialogRef | undefined;

  constructor(
    public dialogService: DialogService,
    private storage: DataService
  ) { }

  public ngOnInit(): void {
    const saved = this.storage.get(StorageKeys.profile_picture);
    if(saved){
      this.currentImage = saved;
    } else {
      this.currentImage = this.defaultImage;
    }
  }

  public onFileSelect(event: any) {
    const imageURL = URL.createObjectURL(event.target.files[0]);
    this.dialogRef = this.dialogService.open(CropperComponent, { header: 'Selecione a região', width: 'fit-content', data: { url: imageURL } });
    
    this.dialogRef.onClose.subscribe((url: string) => {
      if(!url) return;
      this.currentImage = url;
      this.storage.set(StorageKeys.profile_picture, url);
    });
  }

}
