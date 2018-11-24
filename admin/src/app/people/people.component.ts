import { Component, OnInit } from '@angular/core';
import { ApiService } from '../api/api.service';

@Component({
  selector: 'app-people',
  templateUrl: './people.component.html',
  styleUrls: ['./people.component.sass']
})
export class PeopleComponent implements OnInit {
  name: string;
  id: number;
  submitted: boolean = false;
  model: any = {};

  constructor(private api: ApiService) { }

  ngOnInit() {
    this.api.getUser('deon')
      .subscribe((user) => {
        this.name = user.name;
        this.id = user.id;
      });
  }

  createUser(user: any) {
    this.api.createUser(user)
      .subscribe((result) => {
        console.log(result);
      });
  }
}
