class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.63"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.63"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.63/six7-0.7.63-arm64-apple-darwin.tar.gz"
      sha256 "b959bbe957072ee0c67a77d8ca56d6bc748940c93933ad54ba6f283aec4d3a96"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
