import { Component, OnInit } from '@angular/core';
import { ApiService } from '../api/api.service';
import { Observable, pipe } from 'rxjs';
import { take } from 'rxjs/operators';


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
  users: Observable<any[]>;

  constructor(private api: ApiService) { }

  ngOnInit() {
    this.users = this.api.getUsers().pipe(take(1));
  }

  createUser(user: any) {
    this.api.createUser(user)
      .pipe(take(1))
      .subscribe((result) => {
        this.users = this.api.getUsers().pipe(take(1));
      });
  }
}
