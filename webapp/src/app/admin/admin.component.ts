import { Component, OnInit } from '@angular/core';
import { EChartsOption } from "echarts";
import { HttpClient } from "@angular/common/http";
import { share } from "rxjs/operators";
import { Observable } from "rxjs";

type Task = Object;

interface MediaDayStatistic {
  images_count: number;
  images_size_bytes: number;
  videos_count: number;
  videos_size_bytes: number;
}

type MediaStatistics = { [date: string]: MediaDayStatistic };

@Component({
  selector: 'app-stats',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.scss']
})
export class AdminComponent implements OnInit {

  private echartInstance: any;

  chartOption: EChartsOption = {};
  tasks$: Observable<Task>;
  total_images_count: number;
  total_videos_count: number;
  total_files_size: number;

  constructor(
    private http: HttpClient
  ) {
  }

  ngOnInit(): void {
    this.tasks$ = this.http.get('/api/admin/tasks').pipe(share());

    const mediaStatistic$ = this.http.get('/api/admin/media-statistics') as Observable<MediaStatistics>;
    mediaStatistic$
      .subscribe(media_statistic => {
        this.updateBasicNumberStatistics(media_statistic);
        this.updateChartOptions(media_statistic);
      });
  }

  private updateChartOptions(statistics: MediaStatistics) {
    let dates = Object.keys(statistics);
    let image_count_by_date = dates.map(date => [date, statistics[date].images_count]);
    let video_count_by_date = dates.map(date => [date, statistics[date].videos_count]);

    this.chartOption = {
      grid: {
        top: 10,
        left: 50,
        right: 20,
      },
      xAxis: {
        type: 'time',
      },
      yAxis: {
        type: 'value',
      },
      dataZoom: [
        {type: 'inside', start: 0, end: dates.length - 1},
        {start: 0, end: dates.length - 1}
      ],
      tooltip: {
        trigger: 'axis',
      },
      series: [
        {
          data: image_count_by_date,
          type: 'line',
          name: 'Images'
        },
        {
          data: video_count_by_date,
          type: 'line',
          name: 'Videos'
        },
      ],
    };
    if (this.echartInstance) {
      this.echartInstance.setOption(this.chartOption);
    }
  }

  onChartInit(echartInstance: any) {
    this.echartInstance = echartInstance;
  }

  private updateBasicNumberStatistics(statistics: MediaStatistics) {
    this.total_images_count = Object.values(statistics).reduce(
      (acc, statistic) => acc+statistic.images_count,
      0
    );
    this.total_videos_count = Object.values(statistics).reduce(
      (acc, statistic) => acc+statistic.videos_count,
      0
    );
    this.total_files_size = Object.values(statistics).reduce(
      (acc, statistic) => acc+statistic.images_size_bytes+statistic.videos_size_bytes,
      0
    );
  }
}
