use gstreamer as gst;
use gstreamer::prelude::*;


fn main() {
    // 初始化GStreamer
    gst::init().unwrap();

    // 创建一个播放管道
    let pipeline = gst::Pipeline::new(None);

    // 构建管道描述字符串
    // 注意这里直接使用文件路径
    let uri = "file:///mnt/d/dataset/pegnzhou/monitor/2024_11_27_监控导出/灯光误报/W-07管廊监控（纬三路晟源石化南侧）_20241127093704/W-07管廊监控（纬三路晟源石化南侧）_20241126063700-20241126064000_1.mp4";
    // 解析管道描述，并将其设置到管道中
    let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property_from_str("uri", &uri);

    let rtspsrc = gst::ElementFactory::make("rtspsrc", None).unwrap();
    rtspsrc.set_property_from_str("location", &"rtsp://your-rtsp-address");

    if let Err(err) = pipeline.add(&playbin) {
        eprintln!("Failed to add playbin to pipeline: {}", err);
        return;
    }
    
    // 启动管道
    match pipeline.set_state(gst::State::Playing) {
        Ok(_) => println!("Pipeline set to Playing state"),
        Err(err) => {
            eprintln!("Failed to set pipeline to Playing state: {}", err);
            return;
        }
    }

    // 等待直到错误发生或者EOS (End-Of-Stream)
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                break;
            }
            _ => (),
        }
    }

    // 清理
    if let Err(err) = pipeline.set_state(gst::State::Null) {
        eprintln!("Failed to set pipeline to Null state: {}", err);
    }

    println!("正常退出程序");

}