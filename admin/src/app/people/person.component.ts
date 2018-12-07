import { Component, OnInit } from '@angular/core';
import { ApiService } from '../api/api.service';
import { switchMap } from 'rxjs/operators';
import { Router, ActivatedRoute, ParamMap } from '@angular/router';


@Component({
  selector: 'app-person',
  templateUrl: './person.component.html',
  styleUrls: ['./person.component.sass']
})
export class PersonComponent implements OnInit {
  person$: any;

  constructor(  private route: ActivatedRoute,
    private router: Router,
    private api: ApiService
  ) {
  }

  ngOnInit() {
    let id = this.route.snapshot.paramMap.get('id');
    this.person$ = this.api.getUser(id);
    console.log(this.person$)
  }

}