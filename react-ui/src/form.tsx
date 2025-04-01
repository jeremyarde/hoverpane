type WidgetSchema = {
  url: string;
  level: "normal" | "alwaysontop" | "alwaysonbottom";
  title: string;
};

const defaultValues: WidgetSchema = {
  level: "normal",
  url: "",
  title: "",
};

export default function WidgetForm() {
  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data = {
      url: formData.get("url") as string,
      level: formData.get("level") as WidgetSchema["level"],
      title: formData.get("title") as string,
    };
    console.log("Form submitted:", data);

    const res = await fetch("http://127.0.0.1:3000/widgets", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    const widget = await res.json();
    console.log("Widget created:", widget);

    // window.ipc.postMessage(
    //   JSON.stringify({
    //     createwidget: {
    //       url: data.url,
    //       level: data.level,
    //       title: data.title ? data.title : "",
    //       // refresh_interval: data.refresh_interval,
    //     },
    //   })
    // );
  };

  const handleReset = (event: React.FormEvent<HTMLFormElement>) => {
    event.currentTarget.reset();
  };

  const inputClass =
    "w-full px-4 h-8 bg-[#FDFD96] border-[3px] border-black text-lg focus:outline-none appearance-none";
  const labelClass =
    "block w-full bg-[#FF90BC] h-8 leading-7 border-x-[3px] border-t-[3px] border-black text-center font-black text-lg uppercase";

  return (
    <div className="p-2 max-w-md mx-auto">
      <div className="shadow-[6px_6px_0px_0px_rgba(0,0,0,1)]">
        <form onSubmit={handleSubmit} onReset={handleReset}>
          {/* URL Field */}
          <div>
            <label htmlFor="url" className={labelClass}>
              URL
            </label>
            <input
              type="text"
              id="url"
              name="url"
              defaultValue={defaultValues.url}
              placeholder="https://example.com"
              className={`${inputClass} border-b-[3px]`}
              required
            />
          </div>
          <div>
            <label htmlFor="title" className={labelClass}>
              Title
            </label>
            <input
              type="text"
              id="title"
              name="title"
              defaultValue={defaultValues.title}
              placeholder="Title"
              className={`${inputClass} border-b-[3px]`}
              required
            />
          </div>

          {/* Window Level Field */}
          <div>
            <label htmlFor="level" className={labelClass}>
              Window Level
            </label>
            <select
              id="level"
              name="level"
              defaultValue={defaultValues.level}
              className={`${inputClass} border-b-[3px] cursor-pointer`}
            >
              <option value="normal">Normal</option>
              <option value="alwaysontop">Always on Top</option>
              <option value="alwaysonbottom">Always on Bottom</option>
            </select>
          </div>

          {/* Refresh Interval Field */}
          <div>
            {/* <label htmlFor="refresh_interval" className={labelClass}>
              Refresh Interval (ms)
            </label>
            <input
              type="number"
              id="refresh_interval"
              name="refresh_interval"
              defaultValue={defaultValues.refresh_interval}
              min="0"
              placeholder="0"
              className={`${inputClass} border-b-[3px]`}
            /> */}
          </div>

          {/* Form Actions */}
          <div className="flex w-full bg-[#FF90BC] border-b-[3px] border-black">
            <button
              type="reset"
              className="h-10 w-1/3 text-lg font-black bg-[#98EECC] border-l-[3px] border-black uppercase hover:bg-[#7DCCAA] active:bg-[#98EECC] transition-colors"
            >
              Reset
            </button>
            <button
              type="submit"
              className="h-10 w-2/3 text-lg font-black bg-[#A7D2CB] border-l-[3px] border-black uppercase hover:bg-[#86B1AA] active:bg-[#A7D2CB] transition-colors"
            >
              Create
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
