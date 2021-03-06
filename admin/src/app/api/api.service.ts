import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class ApiService {

  constructor(private http: HttpClient) { }

  getUser(name: string): any {
    return this.http.get(`http://localhost:8000/people/${name}`)
  }

  getUserPosts(name: string): any {
    return this.http.get(`http://localhost:8000/people/${name}/posts`)
  }

  getUserComments(name: string): any {
    return this.http.get(`http://localhost:8000/people/${name}/comments`)
  }

  getUsers(): any {
    return this.http.get(`http://localhost:8000/people`)
  }

  createUser(user: any): any {
    return this.http.post('http://localhost:8000/people', user)
  }

  createRedditor(person: string, username: string): any {
    return this.http.post('http://localhost:8000/reddit', {username: username, person: person});
  }

  createTwitter(person: string, username: string): any {
    return this.http.post('http://localhost:8000/twitter', {username: username, person: person});
  }
}
