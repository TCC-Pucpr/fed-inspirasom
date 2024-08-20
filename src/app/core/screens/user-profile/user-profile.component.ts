import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { FileUploadModule } from 'primeng/fileupload';

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

  constructor() {
  }

  public ngOnInit(): void {
    const saved = localStorage.getItem(this.PFP_KEY);
    if(saved){
      this.currentImage = saved;
    } else {
      this.currentImage = this.defaultImage;
    }
  }

  public onFileSelect(event: any) {
    console.log(event.target.files[0]);
    this.currentImage = URL.createObjectURL(event.target.files[0]);

    fetch(this.currentImage)
      .then(response => response.blob())
      .then(blob => {
        const reader = new FileReader();
        reader.onloadend = () => {
          const base64data = reader.result as string;
          this.currentImage = base64data; 
          localStorage.setItem(this.PFP_KEY, base64data);
        };
        reader.readAsDataURL(blob);
      })
      .catch(error => {
        console.error("Error converting blob to base64:", error);
      });
  }

}
